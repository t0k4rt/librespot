#!/bin/sh

echo Spotify event: ---------------------------------------------
echo PLAYER_EVENT: $PLAYER_EVENT
echo TRACK_ID:     $TRACK_ID
echo OLD_TRACK_ID: $OLD_TRACK_ID
echo TRACK_NAME: $TRACK_NAME
echo TRACK_ARTISTS: $TRACK_ARTISTS
echo TRACK_ALBUM: $TRACK_ALBUM

if [ $PLAYER_EVENT == 'start' ]; then 
    echo  "${TRACK_NAME};${TRACK_ARTISTS};${TRACK_ALBUM};${TRACK_DURATION}" >> tracklist.csv
fi

if [ $PLAYER_EVENT == 'change' ]; then 
    echo  "${TRACK_NAME};${TRACK_ARTISTS};${TRACK_ALBUM};${TRACK_DURATION}" >> tracklist.csv
fi