import std.stdio;
import core.sys.posix.dlfcn;

alias uint function() png_access_version_number_t;

void main(){
    auto lib = dlopen("libpng.so".ptr, RTLD_LAZY | RTLD_LOCAL);

    if (lib is null){
        writeln("Broken");
    }else{
        auto png_access_version_number = cast(png_access_version_number_t)dlsym(lib, "png_access_version_number");
        writeln(png_access_version_number());
    }
}
