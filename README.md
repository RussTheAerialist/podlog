# podlog
A s3 log processor for podcasts

This tool takes a directory of files ending in `.log` that are `s3` access logs and outputs a JSON string that represents
downloads of any files that are in the structure of `###.mp3` where the name of the file is an integer.

I wrote this to help collect metrics for my podcast [Batman's Little Bird](http://batmanslittlebird.com) and is my first "real" rust project.
