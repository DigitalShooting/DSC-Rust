{ stdenv, rustPlatform, cmake, pkgconfig, openssl }:

rustPlatform.buildRustPackage rec {
  name = "dsc-${version}";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ openssl pkgconfig ];

  checkPhase = "";
  cargoSha256 = "sha256:1dz36jx380jahvd7jhnd25jw1sxlp231nksvf2f6inqhx9i1nzpx";

  meta = with stdenv.lib; {
    description = "DSC";
    homepage = https://github.com/nuxeh/url-bot-rs;
    license = licenses.isc;
    maintainers = [ maintainers.tailhook ];
    platforms = platforms.all;
  };

  postInstall = ''
    cp -r config/modes/ $out/
  '';
}
