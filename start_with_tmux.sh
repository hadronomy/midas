#!/bin/bash

# Get the current Tmux session name
SESSION=$(tmux display-message -p '#S')
WINDOW="watch"  # Define the window name

# Check if the window already exists in the current session
if ! tmux list-windows -t "$SESSION" | grep -q "$WINDOW"; then
  # Window doesn't exist, so create it in the current session
  tmux new-window -t "$SESSION" -n "$WINDOW"
  echo "Created new Tmux window in session: $SESSION"
else
  # Window exists, kill any running Bubble Tea app process in it
  tmux send-keys -t "$SESSION:$WINDOW" C-c
  echo "Restarting Bubble Tea app in existing Tmux window: $SESSION:$WINDOW"
fi

# Add a small delay to ensure the previous command has completed
sleep 1

# Run Bubble Tea app in the specified Tmux window
tmux send-keys -t "$SESSION:$WINDOW" 'clear; go run cmd/cli/main.go' C-m

# Attach to the current session if running interactively
if [[ $- == *i* ]]; then
  tmux attach-session -t "$SESSION"
fi
