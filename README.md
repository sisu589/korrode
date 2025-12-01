![[korrode.png]]
### **Fast Bcrypt / MD5 / SHA‑256 Cracker (Rust)**
*A tiny command‑line utility written in Rust that can:*

korrode is released for EDUCATIONAL and AUTHORIZED security testing purposes only.

By using this software you agree:
• You will only test systems you own or have explicit permission to test
• You will not use it for any illegal or unauthorized activity
• The authors are not responsible for misuse

Verify bcrypt hashes (any cost factor)
Crack MD5 and SHA‑256 hashes
Run dictionary attacks or numeric brute‑force in parallel (Rayon)
Optionally use mutation rules or GPU‑accelerated hashcat for extra speed
**NOTE – This tool is for authorized security testing only. Never attack systems you don’t own or have explicit permission to audit.**

## Table of Contents
Prerequisites
Installation / Build
Wordlist Management
Usage Examples
.gitignore template
Tips & Tricks
License
Prerequisites
Tool	Minimum version
Rust (cargo + rustc)	1.70 (stable)
Optional – GPU	NVIDIA/AMD driver + hashcat (for bcrypt mode)
Optional – Python	python3 (for the tiny mutator script)
Install Rust via the official installer:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Installation / Build
# Clone the repo (or copy the source into a folder called korrode)
git clone https://github.com/sisu589/korrode.git
cd korrode

# Build the release binary (optimised)
cargo build --release

The executable will appear at:

target/release/korrode
Wordlist Management
Korrode expects a plain‑text file with one candidate per line.

### Get a ready‑made list (rockyou)
curl -L -o wordlists/rockyou.txt \
    https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt


