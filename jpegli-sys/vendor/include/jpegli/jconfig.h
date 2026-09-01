/* Hand-written replacement for libjxl's CMake-generated jconfig.h.
 * Mirrors the values `configure_file(third_party/libjpeg-turbo/jconfig.h.in)`
 * produces when building jpegli-static with the settings this crate uses
 * (JPEGLI_LIBJPEG_LIBRARY_SOVERSION=8, BITS_IN_JSAMPLE=8, no arithmetic
 * coding, no libjpeg-turbo SIMD). See ../../../build.rs.
 */

#define JPEG_LIB_VERSION 80
#define LIBJPEG_TURBO_VERSION 0
#define LIBJPEG_TURBO_VERSION_NUMBER 0
#define BITS_IN_JSAMPLE 8
#define MEM_SRCDST_SUPPORTED 1
