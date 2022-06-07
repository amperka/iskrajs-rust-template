cargo objcopy --release -- -O binary ./target/iskrajs.bin
uf2conv ./target/iskrajs.bin --base 0x08008000 --family 0x57755a57 --output ./target/iskrajs.uf2
cp ./target/iskrajs.uf2 /media/roman/IskraJSBOOT
