{ self
, makeRustPlatform
, lib
, clippy
, runCommand
, pkgs
, enableLint ? false
, enableTests ? false
,
}:
let
  wasmTarget = "wasm32-unknown-unknown";

  rustVersion = "1.66.0";

  rustWithWasmTarget = pkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasmTarget ];
    extensions = [ "rust-src" ];
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };
in
rustPlatformWasm.buildRustPackage ({
  name = "lightning-gui" + lib.optionalString enableLint "-clippy";
  # avoid trigger rebuilds if unrelated files are changed
  src = runCommand "src" { } ''
    install -D ${../../Cargo.toml} $out/Cargo.toml
    install -D ${../../Cargo.lock} $out/Cargo.lock
    install -D ${../../.cargo/config.toml} $out/.cargo/config.toml
    cp -r ${../../src} $out/src
  '';
  cargoLock.lockFile = ../../Cargo.lock;
  cargoLock.outputHashes = {
    "api-0.1.0" = "sha256-iqr/h9jqNUVoLkikeBsQNqhWDSWOyzRJpshugZpnVi0=";
  };
  buildInputs = [ ];
  nativeBuildInputs = [ rustWithWasmTarget ] ++ lib.optionals enableLint [ clippy ];

  doCheck = enableTests;

  meta = with lib; {
    description = "Bitcoin Lightning Web GUI";
    homepage = "https://github.com/kuutamoaps/lightning-gui";
    license = licenses.asl20;
    maintainers = with maintainers; [ ];
    platforms = platforms.unix;
  };
}
  // lib.optionalAttrs enableLint {
  src = runCommand "src" { } ''
    install -D ${../../Cargo.toml} $out/Cargo.toml
    install -D ${../../Cargo.lock} $out/Cargo.lock
    install -D ${../../.cargo/config.toml} $out/.cargo/config.toml
    cp -r ${../../src} $out/src
  '';
  buildPhase = ''
    cargo clippy --all-targets --all-features --no-deps -- -D warnings
    if grep -R 'dbg!' ./src; then
      echo "use of dbg macro found in code!"
      false
    fi
  '';
  installPhase = ''
    touch $out
  '';
})
