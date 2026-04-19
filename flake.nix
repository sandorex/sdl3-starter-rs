{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;

        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };

      # copied from 'android-project/app/build.gradle'
      androidVersion = {
        buildTools = "34.0.0";
        sdk = "35";
        ndk = "28.2.13676358";
      };

      androidComposition = pkgs.androidenv.composeAndroidPackages {
        buildToolsVersions = [ androidVersion.buildTools ];
        platformVersions = [ androidVersion.sdk ];
        ndkVersions = [ androidVersion.ndk ];
        includeNDK = true;
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [
          "x86_64-unknown-linux-gnu"
          "aarch64-linux-android"
          "armv7-linux-androideabi"
          # "x86_64-linux-android" # emulator
        ];
      };

      sdlPkgs = with pkgs; [
        pkg-config
        just
        cmake
        validatePkgConfig
        gcc
        wayland-scanner
        zenity
        libffi
        python313
        patchelf
        vulkan-headers
        vulkan-loader
        libGL
        libusb1
        libayatana-appindicator
        libdrm
        mesa
        wayland
        wayland-protocols
        pipewire
        libpulseaudio
        alsa-lib
        dbus
        libxtst
        libxcb
        libxkbcommon
        libx11
        libxscrnsaver
        libxcursor
        libxext
        libxfixes
        libxi
        libxrandr
      ];
    in
    {
      devShells.${system} = {
        default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            cargo-ndk

            # android
            androidComposition.androidsdk
            androidComposition.platform-tools
            javaPackages.compiler.openjdk17
          ] ++ sdlPkgs;

          # NOTE SDL3 cannot find video device, x11 or wayland without this
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath sdlPkgs;

          ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";

          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_HOME}/build-tools/${androidVersion.buildTools}/aapt2";
        };
      };
    };
}
