mkdir wasm-filters
for d in */ ; do
	cd "$d";
	cargo=$(ls | grep "Cargo.toml")
	if [ "$cargo" != "" ];then
		wasm-pack build --release
		cd pkg
		file=$(ls | grep "wasm$")
		if [ "$file" != "" ]; then
			cp "$file" ../../wasm-filters/"$file"
		fi
		cd ..
	else
		for k in */ ; do 
			cd "$k"
			cargo=$(ls | grep "Cargo.toml")
			if [ "$cargo" != "" ];then
				wasm-pack build --release
				cd pkg
				file=$(ls | grep "wasm$")
				if [ "$file" != "" ]; then
					cp "$file" ../../../wasm-filters/"$file"
				fi
				cd ..
			fi
			cd ..
		done
	fi
	cd ..
done
cd wasm-filters
tar -czvf wasm-filters.tar.gz ./wasm-filters


