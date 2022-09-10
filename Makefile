all:
	cargo build --release && mv target/release/rip .

clean:
	rm -rf release rip

cross_release: clean
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release --target i686-unknown-linux-musl
	cargo build --release --target i686-pc-windows-gnu

	mkdir -p release/rip_x86_64-pc-windows-gnu
	mkdir -p release/rip_x86_64-unknown-linux-musl
	mkdir -p release/rip_i686-unknown-linux-musl
	mkdir -p release/rip_i686-pc-windows-gnu

	cp COPYING release/rip_x86_64-pc-windows-gnu
	cp COPYING release/rip_x86_64-unknown-linux-musl
	cp COPYING release/rip_i686-unknown-linux-musl
	cp COPYING release/rip_i686-pc-windows-gnu

	cp README.md release/rip_x86_64-pc-windows-gnu
	cp README.md release/rip_x86_64-unknown-linux-musl
	cp README.md release/rip_i686-unknown-linux-musl
	cp README.md release/rip_i686-pc-windows-gnu

	cp target/x86_64-pc-windows-gnu/release/rip.exe release/rip_x86_64-pc-windows-gnu
	cp target/x86_64-unknown-linux-musl/release/rip release/rip_x86_64-unknown-linux-musl
	cp target/i686-unknown-linux-musl/release/rip release/rip_i686-unknown-linux-musl
	cp target/i686-pc-windows-gnu/release/rip.exe release/rip_i686-pc-windows-gnu

	cd release && \
	zip -r rip_x86_64-pc-windows-gnu rip_x86_64-pc-windows-gnu/ && \
	zip -r rip_x86_64-unknown-linux-musl rip_x86_64-unknown-linux-musl && \
	zip -r rip_i686-unknown-linux-musl rip_i686-unknown-linux-musl && \
	zip -r rip_i686-pc-windows-gnu rip_i686-pc-windows-gnu

	rm -r release/rip_x86_64-pc-windows-gnu
	rm -r release/rip_x86_64-unknown-linux-musl
	rm -r release/rip_i686-unknown-linux-musl
	rm -r release/rip_i686-pc-windows-gnu