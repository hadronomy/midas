use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use file_type_enum::FileType;
use miette::*;

pub type TrieMap<Extra> = BTreeMap<PathBuf, FsTree<Extra>>;

#[derive(Debug)]
pub enum FsTree<Metadata> {
    File { metadata: Metadata },
    Directory { metadata: Metadata, children: TrieMap<Metadata> },
    Symlink(PathBuf),
}

impl<Metadata> FsTree<Metadata> {
    /// Creates a new file node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use midas::fs_tree::FsTree;
    ///
    /// let tree = FsTree::new_file(());
    /// ```
    pub fn new_file(metadata: Metadata) -> Self {
        Self::File { metadata }
    }

    /// Creates a new directory node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use midas::fs_tree::FsTree;
    ///
    /// let tree = FsTree::new_dir(());
    /// ```
    pub fn new_dir(metadata: Metadata) -> Self {
        Self::Directory { metadata, children: TrieMap::new() }
    }

    /// Returns the length of the tree, not counting the root node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use midas::fs_tree::FsTree;
    ///
    /// // Empty directory
    /// let tree = FsTree::new_dir(());
    /// assert_eq!(tree.len(), 0);
    ///
    /// // Single file
    /// let tree = FsTree::new_file(());
    /// assert_eq!(tree.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Self::File { .. } => 0,
            Self::Directory { children, .. } => children.len(),
            Self::Symlink(_) => 0,
        }
    }

    /// Returns `true` if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use midas::fs_tree::FsTree;
    ///
    /// // Empty directory
    /// let tree = FsTree::new_dir(());
    /// assert!(tree.is_empty());
    ///
    /// // Single file
    /// let tree = FsTree::new_file(());
    /// assert!(tree.is_empty());
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn read_at<PathRef, MetadataFn>(
        path: PathRef,
        follow_symlinks: bool,
        metadata_fn: MetadataFn,
    ) -> Result<Self>
    where
        PathRef: AsRef<Path> + Clone,
        MetadataFn: Fn(&Path) -> Result<Metadata>,
    {
        let read_at = if follow_symlinks { FileType::read_at } else { FileType::symlink_read_at };
        let metadata = metadata_fn(path.as_ref())?;

        match read_at(path.clone()).into_diagnostic().wrap_err("Failed to read file type")? {
            FileType::Regular => Ok(Self::File { metadata }),
            FileType::Directory => {
                let mut children = TrieMap::new();
                for entry in std::fs::read_dir(path)
                    .into_diagnostic()
                    .wrap_err("Failed to read directory")?
                {
                    let entry =
                        entry.into_diagnostic().wrap_err("Failed to read directory entry")?;
                    let path = entry.path();
                    let child = Self::read_at(path.clone(), follow_symlinks, &metadata_fn)?;
                    children.insert(path, child);
                }
                Ok(Self::Directory { metadata, children })
            }
            FileType::Symlink => todo!(),
            FileType::BlockDevice => todo!(),
            FileType::CharDevice => todo!(),
            FileType::Fifo => todo!(),
            FileType::Socket => todo!(),
        }
    }

    pub fn insert(&mut self, path: PathBuf, node: FsTree<Metadata>) -> Result<()> {
        if let Self::Directory { children, .. } = self {
            children.insert(path, node).wrap_err("Failed to insert node")?;
            return Ok(());
        }
        Err(miette!("Cannot insert into a non-directory node"))
    }

    /// Returns `true` if self matches the [`FsTree::File`] variant.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    /// Returns `true` if self matches the [`FsTree::Directory`] variant.
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory { .. })
    }

    /// Returns `true` if self matches the [`FsTree::Symlink`] variant.
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink(_))
    }
}

pub trait IntoFsTree<Metadata> {
    fn into_fs_tree(self) -> FsTree<Metadata>;
}

pub trait IntoFsTreeWithMetadata<Metadata, MetadataFn>
where
    MetadataFn: Fn(&Path) -> Result<Metadata>,
{
    fn into_fs_tree_with_metadata(self, metadata_fn: MetadataFn) -> FsTree<Metadata>;
}

impl<Metadata, T> IntoFsTree<Metadata> for T
where
    Metadata: Default,
    T: IntoFsTreeWithMetadata<Metadata, fn(&Path) -> Result<Metadata>>,
{
    fn into_fs_tree(self) -> FsTree<Metadata> {
        self.into_fs_tree_with_metadata(|_| Ok(Metadata::default()))
    }
}

impl<Metadata> IntoFsTreeWithMetadata<Metadata, fn(&Path) -> Result<Metadata>> for PathBuf {
    fn into_fs_tree_with_metadata(
        self,
        metadata_fn: fn(&Path) -> Result<Metadata>,
    ) -> FsTree<Metadata> {
        FsTree::read_at(self, true, metadata_fn).unwrap()
    }
}

impl<Metadata> IntoFsTreeWithMetadata<Metadata, fn(&Path) -> Result<Metadata>> for &Path {
    fn into_fs_tree_with_metadata(
        self,
        metadata_fn: fn(&Path) -> Result<Metadata>,
    ) -> FsTree<Metadata> {
        FsTree::read_at(self, true, metadata_fn).unwrap()
    }
}

impl<Metadata> IntoFsTreeWithMetadata<Metadata, fn(&Path) -> Result<Metadata>>
    for walkdir::WalkDir
{
    fn into_fs_tree_with_metadata(
        self,
        metadata_fn: fn(&Path) -> Result<Metadata>,
    ) -> FsTree<Metadata> {
        FsTree::read_at(self.into_iter().next().unwrap().unwrap().path(), true, metadata_fn)
            .unwrap()
    }
}
