{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    lld
  ];

  LD_LIBRARY_PATH = with pkgs;
    lib.makeLibraryPath [
      openssl
    ];
}
