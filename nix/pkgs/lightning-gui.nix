{ self
, makeRustPlatform
, lib
, clippy
, pkg-config
, openssl
, trunk
, wasm-bindgen-cli
, binaryen
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
    install -D ${../../index.html} $out/index.html
    install -D ${../../404.html} $out/404.html
    cp -r ${../../src} $out/src
  '';
  cargoLock.lockFile = ../../Cargo.lock;
  cargoLock.outputHashes = {
    "api-0.1.0" = "sha256-1/8yYINoNaP1x4D7fxv9wwCHksECKfKSKvRHQ9TKi0Y=";
  };
  buildInputs = [ openssl ];
  nativeBuildInputs = [ rustWithWasmTarget pkg-config trunk wasm-bindgen-cli binaryen ] ++ lib.optionals enableLint [ clippy ];

  doCheck = enableTests;

  buildPhase = ''
    trunk build --release --public-url /lightning-ui
  '';

  installPhase = ''
    runHook preInstall
    cp -r dist $out
    runHook postInstall
  '';

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
    install -D ${../../index.html} $out/index.html
    install -D ${../../404.html} $out/404.html
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
