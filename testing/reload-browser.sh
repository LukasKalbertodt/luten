#!/bin/sh

CUR_WID=$(xdotool getwindowfocus)
XWID=$(xdotool search --name 'Testatverwaltung - Chromium')
xdotool windowactivate $XWID
xdotool key 'ctrl+r'
xdotool windowactivate $CUR_WID
