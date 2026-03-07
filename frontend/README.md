### Ensure you have the required OS prereqs by running:
```sh
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Then:
```sh
npm install
npm install -D @tauri-apps/cli
npm install @tauri-apps/api
```

### Then run:
```sh
npx tauri dev
```

Will containerize