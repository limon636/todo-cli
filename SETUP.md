# 🚀 Setup Instructions for Curl Install

Follow these steps to enable `curl -fsSL https://raw.githubusercontent.com/limon636/todo-cli/main/install.sh | bash`

## 1. 📤 Push to GitHub Repository

First, make sure all files are committed and pushed to your GitHub repository:

```bash
# Add all new files
git add .

# Commit changes
git commit -m "Add curl installer and GitHub Actions release workflow"

# Push to main branch
git push origin main
```

## 2. 🏷️ Create a Release (Optional but Recommended)

To enable pre-built binaries and trigger the release workflow:

```bash
# Tag a new version
git tag v0.1.0

# Push the tag
git push origin v0.1.0
```

This will trigger the GitHub Actions workflow to build binaries for multiple platforms.

## 3. 🧪 Test the Installer

Once pushed, test the installer:

```bash
# Test the curl install command
curl -fsSL https://raw.githubusercontent.com/limon636/todo-cli/main/install.sh | bash
```

## 4. 📁 Repository Setup Checklist

Make sure your GitHub repository has these files:

- ✅ `install.sh` - The installer script
- ✅ `.github/workflows/release.yml` - Automated releases
- ✅ `README.md` - Updated with curl install command
- ✅ Source code with version support

## 5. 🔧 Local Testing

You can test the installer locally first:

```bash
# Test locally before pushing
./test_install.sh
```

## 6. 📋 What the Installer Does

The `install.sh` script will:

1. **Auto-detect** user's OS and architecture
2. **Install Rust** (if not already installed)
3. **Clone** your repository
4. **Compile** the release binary
5. **Install** to `~/.local/bin/todo`
6. **Update PATH** in shell profile
7. **Verify** installation

## 7. 🌍 Supported Platforms

The installer supports:

- **Linux**: x86_64, aarch64, armv7
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64 (via Git Bash/WSL)

## 8. 📢 Share with Users

Once everything is set up, users can install with just:

```bash
curl -fsSL https://raw.githubusercontent.com/limon636/todo-cli/main/install.sh | bash
```

## 🎉 That's it!

Your todo CLI now has professional-grade installation experience!

---

## 🐛 Troubleshooting

If users have issues:

1. **Missing curl/wget**: Install either curl or wget
2. **No Rust**: The script will install it automatically
3. **Permission issues**: Make sure `~/.local/bin` is writable
4. **PATH issues**: Restart terminal or run `source ~/.bashrc`

## 🔄 Future Improvements

- Add pre-built binary downloads for faster installs
- Add uninstall script
- Add update mechanism
- Add package manager support (homebrew, apt, etc.)