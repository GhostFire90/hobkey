cd out

rm boot.img
fallocate -l 50m boot.img

<<comment
g : create new gpt table
n : new partition, all the newlines are for all the defaults
t : set the type
1 : type to efi
W : write
comment

printf "g\nn\n\n\n\nt\n1\nw\n" | fdisk boot.img
sudo mkfs.fat -F32 boot.img

tdir=$(mktemp -d)

sudo mount boot.img $tdir
sudo rm -rf $tdir/*




finaldir=$tdir/EFI/BOOT


sudo mkdir -p $finaldir
sudo mkdir -p $tdir/files


sudo cp BOOTX64.efi $finaldir
sudo cp -r -H ../out/kernel.bin $tdir/files/
sudo cp -r -H ../out/initrd.tar $tdir/files/
#sudo cp ../out/test.efi $minedir/test.efi

sudo umount $tdir
rm -r $tdir