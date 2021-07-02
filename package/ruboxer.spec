%global appname ruboxer
%global version 0.1.0

Name: %{appname}
Version: %{version}
Release: 1%{?dist}
Summary: Rudimentary container tool for Linux
License: MIT
URL: https://github.com/DonaldKellett/%{appname}
Source0: https://github.com/DonaldKellett/%{appname}/archive/refs/tags/v%{version}.tar.gz
BuildRequires: cargo

%description
A rudimentary tool for running containers from unpacked images in Linux.

%prep
%setup -q

%build
cargo build

%install
mkdir -p %{buildroot}/%{_bindir}
cp target/debug/%{appname} %{buildroot}/%{_bindir}/%{appname}
mkdir -p %{buildroot}/%{_mandir}/man8
cp %{appname}.8 %{buildroot}/%{_mandir}/man8/%{appname}.8

%files
%{_bindir}/%{appname}
%{_mandir}/man8/%{appname}.8.gz

%changelog
* Fri Jul 2 2021 Donald Sebastian Leung <donaldsebleung@gmail.com> - 0.1.0-1
- Initial release
