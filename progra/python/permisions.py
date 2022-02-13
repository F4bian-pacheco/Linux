
"""
The permisions of a file in a Linux system are split into three sets
 of three permisions: read, write and execute for the owner, group
 and others. Each of the three values can be expressed as an octal
 number summing each permission, with 4 corresponding to read, 2 to
 write and 1 to execute. Or it can be written with a string using
 the letters r, w and x or - when the permission is not granted.
 For example:

 640 is read/write for the owner, read for the group and no permisions
 for the others; converted to a string, it would be: "rw-r-----"

 755 is read/write/execute for the owner, read/execute for the group
 and others; converted to a string, it would be:"rwxr-xr-x"
"""
