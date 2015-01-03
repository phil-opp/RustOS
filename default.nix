(import <nixpkgs> {}).callPackage (
{stdenv, nasm, qemu, gdb}: stdenv.mkDerivation rec {
  name = "RustOS";

  nativeBuildInputs = [ gdb nasm qemu ];
  src = ./src;

  enableParallelBuilding = true;
}) {}
