{
  rustPlatform,
  lib,
}:
rustPlatform.buildRustPackage {
  pname = "vin";
  version = "0.1.0";

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  buildInputs = [
  ];

  meta = {
    description = "Interpreter for the vin scripting language";
    homepage = "https://github.com/myume/vin";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [myu];
    platforms = lib.platforms.linux;
  };
}
