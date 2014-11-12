/** @file aiFileIO.h
 *  @brief Defines generic C routines to access memory-mapped files
 */

struct aiFileIO;
struct aiFile;

// aiFile callbacks
typedef size_t   (*aiFileWriteProc) (C_STRUCT aiFile*,   const char*, size_t, size_t);
typedef size_t   (*aiFileReadProc)  (C_STRUCT aiFile*,   char*, size_t,size_t);
typedef size_t   (*aiFileTellProc)  (C_STRUCT aiFile*);
typedef void     (*aiFileFlushProc) (C_STRUCT aiFile*);
typedef aiReturn (*aiFileSeek)(C_STRUCT aiFile*, size_t, aiOrigin);

// aiFileIO callbacks
typedef aiFile* (*aiFileOpenProc)  (C_STRUCT aiFileIO*, const char*, const char*);
typedef void    (*aiFileCloseProc) (C_STRUCT aiFileIO*, C_STRUCT aiFile*);

// Represents user-defined data
typedef char* aiUserData;

// ----------------------------------------------------------------------------------
/** @brief C-API: File system callbacks
    *
    *  Provided are functions to open and close files. Supply a custom structure to
    *  the import function. If you don't, a default implementation is used. Use custom
    *  file systems to enable reading from other sources, such as ZIPs 
    *  or memory locations. */
struct aiFileIO
{
    /** Function used to open a new file
    */
    aiFileOpenProc OpenProc;

    /** Function used to close an existing file
    */
    aiFileCloseProc CloseProc;

    /** User-defined, opaque data */
    aiUserData UserData;
};

// ----------------------------------------------------------------------------------
/** @brief C-API: File callbacks
    *
    *  Actually, it's a data structure to wrap a set of fXXXX (e.g fopen) 
    *  replacement functions.
    *
    *  The default implementation of the functions utilizes the fXXX functions from 
    *  the CRT. However, you can supply a custom implementation to Assimp by
    *  delivering a custom aiFileIO. Use this to enable reading from other sources, 
    *  such as ZIP archives or memory locations. */
struct aiFile
{
    /** Callback to read from a file */
    aiFileReadProc ReadProc;

    /** Callback to write to a file */
    aiFileWriteProc WriteProc;

    /** Callback to retrieve the current position of 
        *  the file cursor (ftell())
        */
    aiFileTellProc TellProc;

    /** Callback to retrieve the size of the file, 
        *  in bytes
        */
    aiFileTellProc FileSizeProc;

    /** Callback to set the current position
        * of the file cursor (fseek())
        */
    aiFileSeek SeekProc;

    /** Callback to flush the file contents
    */
    aiFileFlushProc FlushProc;

    /** User-defined, opaque data
    */
    aiUserData UserData;
};

}
