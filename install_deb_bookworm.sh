#!/bin/bash
set -x
set -euo pipefail

arch="${1}"
shift

# need to install certain local dependencies
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install --assume-yes --no-install-recommends \
	ca-certificates \
	curl \
	cpio \
	sharutils \
	gnupg

# get our debian sources
debsource="deb http://http.debian.net/debian/ bookworm main"
debsource="${debsource}\ndeb http://security.debian.org/ bookworm/updates main"

# temporarily use debian sources rather than ubuntu.
mv /etc/apt/sources.list /etc/apt/sources.list.bak
echo -e "${debsource}" >/etc/apt/sources.list

# Update architecture handling
dpkg --add-architecture "${arch}" || echo "foreign-architecture ${arch}" \
	>/etc/dpkg/dpkg.cfg.d/multiarch

# Add Bookworm keys.
curl --retry 3 -sSfL 'https://ftp-master.debian.org/keys/archive-key-{11,12}.asc' -O
curl --retry 3 -sSfL 'https://ftp-master.debian.org/keys/release-{11,12}.asc' -O

for key in *.asc *.key; do
	apt-key add "${key}"
	rm "${key}"
done

# allow apt-get to retry downloads
echo 'APT::Acquire::Retries "3";' >/etc/apt/apt.conf.d/80-retries

apt-get update
for dep in $@; do
	apt-get install "${dep}:${arch}" --assume-yes
done

# Restore our old sources list
mv -f /etc/apt/sources.list.bak /etc/apt/sources.list
if [ -f /etc/dpkg/dpkg.cfg.d/multiarch.bak ]; then
	mv /etc/dpkg/dpkg.cfg.d/multiarch.bak /etc/dpkg/dpkg.cfg.d/multiarch
fi

# can fail if arch is used (amd64 and/or i386)
dpkg --remove-architecture "${arch}" || true
apt-get update
