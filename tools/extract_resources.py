from pathlib import Path
import struct

def unpack(path: str, out: str):
    data=Path(path).read_bytes(); pos=0
    count=struct.unpack_from(">H",data,pos)[0]; pos+=2
    lens=[struct.unpack_from(">H",data,pos+2*i)[0] for i in range(count)]
    pos += 2*count
    out=Path(out); out.mkdir(parents=True,exist_ok=True)
    for i,n in enumerate(lens):
        (out/f"{i:03}.bin").write_bytes(data[pos:pos+n]); pos+=n

if __name__=="__main__":
    unpack("assets/raw/b","assets/chunks/b")
    print("done")
