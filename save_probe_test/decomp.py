import gzip, sys
path=sys.argv[1]
data=open(path,'rb').read()
gz=data.find(b'\x1f\x8b\x08')
print("gzip member starts at file offset", gz, "file size", len(data))
raw=gzip.decompress(data[gz:])
print("decompressed size", len(raw))
out=path+".raw"
open(out,'wb').write(raw)
print("wrote", out)
