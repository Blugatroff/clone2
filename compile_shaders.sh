glslc -fshader-stage=vertex src/chunk_middle_ware/shadow.glsl -o src/chunk_middle_ware/shadow.glsl.spv -O
glslc -fshader-stage=vertex src/chunk_middle_ware/vs.glsl -o src/chunk_middle_ware/vs.glsl.spv -O
glslc -fshader-stage=fragment src/chunk_middle_ware/fs.glsl -o src/chunk_middle_ware/fs.glsl.spv -O

glslc -fshader-stage=vertex src/flat_middleware/vs.glsl -o src/flat_middleware/vs.glsl.spv -O
glslc -fshader-stage=fragment src/flat_middleware/fs.glsl -o src/flat_middleware/fs.glsl.spv -O