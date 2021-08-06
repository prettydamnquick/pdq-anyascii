pub fn block(block_num: u16) -> &'static [u8] {
	match block_num {
		0x000 => include_bytes!("data/000.bin"),
		0x001 => include_bytes!("data/001.bin"),
		0x002 => include_bytes!("data/002.bin"),
		0x003 => include_bytes!("data/003.bin"),
		0x004 => include_bytes!("data/004.bin"),
		0x005 => include_bytes!("data/005.bin"),
		0x006 => include_bytes!("data/006.bin"),
		0x007 => include_bytes!("data/007.bin"),
		0x008 => include_bytes!("data/008.bin"),
		0x009 => include_bytes!("data/009.bin"),
		0x00a => include_bytes!("data/00a.bin"),
		0x00b => include_bytes!("data/00b.bin"),
		0x00c => include_bytes!("data/00c.bin"),
		0x00d => include_bytes!("data/00d.bin"),
		0x00e => include_bytes!("data/00e.bin"),
		0x00f => include_bytes!("data/00f.bin"),
		0x010 => include_bytes!("data/010.bin"),
		0x011 => include_bytes!("data/011.bin"),
		0x012 => include_bytes!("data/012.bin"),
		0x013 => include_bytes!("data/013.bin"),
		0x014 => include_bytes!("data/014.bin"),
		0x015 => include_bytes!("data/015.bin"),
		0x016 => include_bytes!("data/016.bin"),
		0x017 => include_bytes!("data/017.bin"),
		0x018 => include_bytes!("data/018.bin"),
		0x019 => include_bytes!("data/019.bin"),
		0x01a => include_bytes!("data/01a.bin"),
		0x01b => include_bytes!("data/01b.bin"),
		0x01c => include_bytes!("data/01c.bin"),
		0x01d => include_bytes!("data/01d.bin"),
		0x01e => include_bytes!("data/01e.bin"),
		0x01f => include_bytes!("data/01f.bin"),
		0x020 => include_bytes!("data/020.bin"),
		0x021 => include_bytes!("data/021.bin"),
		0x022 => include_bytes!("data/022.bin"),
		0x023 => include_bytes!("data/023.bin"),
		0x024 => include_bytes!("data/024.bin"),
		0x025 => include_bytes!("data/025.bin"),
		0x026 => include_bytes!("data/026.bin"),
		0x027 => include_bytes!("data/027.bin"),
		0x028 => include_bytes!("data/028.bin"),
		0x029 => include_bytes!("data/029.bin"),
		0x02a => include_bytes!("data/02a.bin"),
		0x02b => include_bytes!("data/02b.bin"),
		0x02c => include_bytes!("data/02c.bin"),
		0x02d => include_bytes!("data/02d.bin"),
		0x02e => include_bytes!("data/02e.bin"),
		0x02f => include_bytes!("data/02f.bin"),
		0x030 => include_bytes!("data/030.bin"),
		0x031 => include_bytes!("data/031.bin"),
		0x032 => include_bytes!("data/032.bin"),
		0x033 => include_bytes!("data/033.bin"),
		0x034 => include_bytes!("data/034.bin"),
		0x035 => include_bytes!("data/035.bin"),
		0x036 => include_bytes!("data/036.bin"),
		0x037 => include_bytes!("data/037.bin"),
		0x038 => include_bytes!("data/038.bin"),
		0x039 => include_bytes!("data/039.bin"),
		0x03a => include_bytes!("data/03a.bin"),
		0x03b => include_bytes!("data/03b.bin"),
		0x03c => include_bytes!("data/03c.bin"),
		0x03d => include_bytes!("data/03d.bin"),
		0x03e => include_bytes!("data/03e.bin"),
		0x03f => include_bytes!("data/03f.bin"),
		0x040 => include_bytes!("data/040.bin"),
		0x041 => include_bytes!("data/041.bin"),
		0x042 => include_bytes!("data/042.bin"),
		0x043 => include_bytes!("data/043.bin"),
		0x044 => include_bytes!("data/044.bin"),
		0x045 => include_bytes!("data/045.bin"),
		0x046 => include_bytes!("data/046.bin"),
		0x047 => include_bytes!("data/047.bin"),
		0x048 => include_bytes!("data/048.bin"),
		0x049 => include_bytes!("data/049.bin"),
		0x04a => include_bytes!("data/04a.bin"),
		0x04b => include_bytes!("data/04b.bin"),
		0x04c => include_bytes!("data/04c.bin"),
		0x04d => include_bytes!("data/04d.bin"),
		0x04e => include_bytes!("data/04e.bin"),
		0x04f => include_bytes!("data/04f.bin"),
		0x050 => include_bytes!("data/050.bin"),
		0x051 => include_bytes!("data/051.bin"),
		0x052 => include_bytes!("data/052.bin"),
		0x053 => include_bytes!("data/053.bin"),
		0x054 => include_bytes!("data/054.bin"),
		0x055 => include_bytes!("data/055.bin"),
		0x056 => include_bytes!("data/056.bin"),
		0x057 => include_bytes!("data/057.bin"),
		0x058 => include_bytes!("data/058.bin"),
		0x059 => include_bytes!("data/059.bin"),
		0x05a => include_bytes!("data/05a.bin"),
		0x05b => include_bytes!("data/05b.bin"),
		0x05c => include_bytes!("data/05c.bin"),
		0x05d => include_bytes!("data/05d.bin"),
		0x05e => include_bytes!("data/05e.bin"),
		0x05f => include_bytes!("data/05f.bin"),
		0x060 => include_bytes!("data/060.bin"),
		0x061 => include_bytes!("data/061.bin"),
		0x062 => include_bytes!("data/062.bin"),
		0x063 => include_bytes!("data/063.bin"),
		0x064 => include_bytes!("data/064.bin"),
		0x065 => include_bytes!("data/065.bin"),
		0x066 => include_bytes!("data/066.bin"),
		0x067 => include_bytes!("data/067.bin"),
		0x068 => include_bytes!("data/068.bin"),
		0x069 => include_bytes!("data/069.bin"),
		0x06a => include_bytes!("data/06a.bin"),
		0x06b => include_bytes!("data/06b.bin"),
		0x06c => include_bytes!("data/06c.bin"),
		0x06d => include_bytes!("data/06d.bin"),
		0x06e => include_bytes!("data/06e.bin"),
		0x06f => include_bytes!("data/06f.bin"),
		0x070 => include_bytes!("data/070.bin"),
		0x071 => include_bytes!("data/071.bin"),
		0x072 => include_bytes!("data/072.bin"),
		0x073 => include_bytes!("data/073.bin"),
		0x074 => include_bytes!("data/074.bin"),
		0x075 => include_bytes!("data/075.bin"),
		0x076 => include_bytes!("data/076.bin"),
		0x077 => include_bytes!("data/077.bin"),
		0x078 => include_bytes!("data/078.bin"),
		0x079 => include_bytes!("data/079.bin"),
		0x07a => include_bytes!("data/07a.bin"),
		0x07b => include_bytes!("data/07b.bin"),
		0x07c => include_bytes!("data/07c.bin"),
		0x07d => include_bytes!("data/07d.bin"),
		0x07e => include_bytes!("data/07e.bin"),
		0x07f => include_bytes!("data/07f.bin"),
		0x080 => include_bytes!("data/080.bin"),
		0x081 => include_bytes!("data/081.bin"),
		0x082 => include_bytes!("data/082.bin"),
		0x083 => include_bytes!("data/083.bin"),
		0x084 => include_bytes!("data/084.bin"),
		0x085 => include_bytes!("data/085.bin"),
		0x086 => include_bytes!("data/086.bin"),
		0x087 => include_bytes!("data/087.bin"),
		0x088 => include_bytes!("data/088.bin"),
		0x089 => include_bytes!("data/089.bin"),
		0x08a => include_bytes!("data/08a.bin"),
		0x08b => include_bytes!("data/08b.bin"),
		0x08c => include_bytes!("data/08c.bin"),
		0x08d => include_bytes!("data/08d.bin"),
		0x08e => include_bytes!("data/08e.bin"),
		0x08f => include_bytes!("data/08f.bin"),
		0x090 => include_bytes!("data/090.bin"),
		0x091 => include_bytes!("data/091.bin"),
		0x092 => include_bytes!("data/092.bin"),
		0x093 => include_bytes!("data/093.bin"),
		0x094 => include_bytes!("data/094.bin"),
		0x095 => include_bytes!("data/095.bin"),
		0x096 => include_bytes!("data/096.bin"),
		0x097 => include_bytes!("data/097.bin"),
		0x098 => include_bytes!("data/098.bin"),
		0x099 => include_bytes!("data/099.bin"),
		0x09a => include_bytes!("data/09a.bin"),
		0x09b => include_bytes!("data/09b.bin"),
		0x09c => include_bytes!("data/09c.bin"),
		0x09d => include_bytes!("data/09d.bin"),
		0x09e => include_bytes!("data/09e.bin"),
		0x09f => include_bytes!("data/09f.bin"),
		0x0a0 => include_bytes!("data/0a0.bin"),
		0x0a1 => include_bytes!("data/0a1.bin"),
		0x0a2 => include_bytes!("data/0a2.bin"),
		0x0a3 => include_bytes!("data/0a3.bin"),
		0x0a4 => include_bytes!("data/0a4.bin"),
		0x0a5 => include_bytes!("data/0a5.bin"),
		0x0a6 => include_bytes!("data/0a6.bin"),
		0x0a7 => include_bytes!("data/0a7.bin"),
		0x0a8 => include_bytes!("data/0a8.bin"),
		0x0a9 => include_bytes!("data/0a9.bin"),
		0x0aa => include_bytes!("data/0aa.bin"),
		0x0ab => include_bytes!("data/0ab.bin"),
		0x0ac => include_bytes!("data/0ac.bin"),
		0x0ad => include_bytes!("data/0ad.bin"),
		0x0ae => include_bytes!("data/0ae.bin"),
		0x0af => include_bytes!("data/0af.bin"),
		0x0b0 => include_bytes!("data/0b0.bin"),
		0x0b1 => include_bytes!("data/0b1.bin"),
		0x0b2 => include_bytes!("data/0b2.bin"),
		0x0b3 => include_bytes!("data/0b3.bin"),
		0x0b4 => include_bytes!("data/0b4.bin"),
		0x0b5 => include_bytes!("data/0b5.bin"),
		0x0b6 => include_bytes!("data/0b6.bin"),
		0x0b7 => include_bytes!("data/0b7.bin"),
		0x0b8 => include_bytes!("data/0b8.bin"),
		0x0b9 => include_bytes!("data/0b9.bin"),
		0x0ba => include_bytes!("data/0ba.bin"),
		0x0bb => include_bytes!("data/0bb.bin"),
		0x0bc => include_bytes!("data/0bc.bin"),
		0x0bd => include_bytes!("data/0bd.bin"),
		0x0be => include_bytes!("data/0be.bin"),
		0x0bf => include_bytes!("data/0bf.bin"),
		0x0c0 => include_bytes!("data/0c0.bin"),
		0x0c1 => include_bytes!("data/0c1.bin"),
		0x0c2 => include_bytes!("data/0c2.bin"),
		0x0c3 => include_bytes!("data/0c3.bin"),
		0x0c4 => include_bytes!("data/0c4.bin"),
		0x0c5 => include_bytes!("data/0c5.bin"),
		0x0c6 => include_bytes!("data/0c6.bin"),
		0x0c7 => include_bytes!("data/0c7.bin"),
		0x0c8 => include_bytes!("data/0c8.bin"),
		0x0c9 => include_bytes!("data/0c9.bin"),
		0x0ca => include_bytes!("data/0ca.bin"),
		0x0cb => include_bytes!("data/0cb.bin"),
		0x0cc => include_bytes!("data/0cc.bin"),
		0x0cd => include_bytes!("data/0cd.bin"),
		0x0ce => include_bytes!("data/0ce.bin"),
		0x0cf => include_bytes!("data/0cf.bin"),
		0x0d0 => include_bytes!("data/0d0.bin"),
		0x0d1 => include_bytes!("data/0d1.bin"),
		0x0d2 => include_bytes!("data/0d2.bin"),
		0x0d3 => include_bytes!("data/0d3.bin"),
		0x0d4 => include_bytes!("data/0d4.bin"),
		0x0d5 => include_bytes!("data/0d5.bin"),
		0x0d6 => include_bytes!("data/0d6.bin"),
		0x0d7 => include_bytes!("data/0d7.bin"),
		0x0f9 => include_bytes!("data/0f9.bin"),
		0x0fa => include_bytes!("data/0fa.bin"),
		0x0fb => include_bytes!("data/0fb.bin"),
		0x0fc => include_bytes!("data/0fc.bin"),
		0x0fd => include_bytes!("data/0fd.bin"),
		0x0fe => include_bytes!("data/0fe.bin"),
		0x0ff => include_bytes!("data/0ff.bin"),
		0x100 => include_bytes!("data/100.bin"),
		0x101 => include_bytes!("data/101.bin"),
		0x102 => include_bytes!("data/102.bin"),
		0x103 => include_bytes!("data/103.bin"),
		0x104 => include_bytes!("data/104.bin"),
		0x105 => include_bytes!("data/105.bin"),
		0x106 => include_bytes!("data/106.bin"),
		0x107 => include_bytes!("data/107.bin"),
		0x108 => include_bytes!("data/108.bin"),
		0x109 => include_bytes!("data/109.bin"),
		0x10a => include_bytes!("data/10a.bin"),
		0x10b => include_bytes!("data/10b.bin"),
		0x10c => include_bytes!("data/10c.bin"),
		0x10d => include_bytes!("data/10d.bin"),
		0x10e => include_bytes!("data/10e.bin"),
		0x10f => include_bytes!("data/10f.bin"),
		0x110 => include_bytes!("data/110.bin"),
		0x111 => include_bytes!("data/111.bin"),
		0x112 => include_bytes!("data/112.bin"),
		0x113 => include_bytes!("data/113.bin"),
		0x114 => include_bytes!("data/114.bin"),
		0x115 => include_bytes!("data/115.bin"),
		0x116 => include_bytes!("data/116.bin"),
		0x117 => include_bytes!("data/117.bin"),
		0x118 => include_bytes!("data/118.bin"),
		0x119 => include_bytes!("data/119.bin"),
		0x11a => include_bytes!("data/11a.bin"),
		0x11c => include_bytes!("data/11c.bin"),
		0x11d => include_bytes!("data/11d.bin"),
		0x11e => include_bytes!("data/11e.bin"),
		0x11f => include_bytes!("data/11f.bin"),
		0x124 => include_bytes!("data/124.bin"),
		0x130 => include_bytes!("data/130.bin"),
		0x131 => include_bytes!("data/131.bin"),
		0x132 => include_bytes!("data/132.bin"),
		0x133 => include_bytes!("data/133.bin"),
		0x134 => include_bytes!("data/134.bin"),
		0x144 => include_bytes!("data/144.bin"),
		0x145 => include_bytes!("data/145.bin"),
		0x146 => include_bytes!("data/146.bin"),
		0x16a => include_bytes!("data/16a.bin"),
		0x16b => include_bytes!("data/16b.bin"),
		0x16e => include_bytes!("data/16e.bin"),
		0x16f => include_bytes!("data/16f.bin"),
		0x170 => include_bytes!("data/170.bin"),
		0x171 => include_bytes!("data/171.bin"),
		0x172 => include_bytes!("data/172.bin"),
		0x173 => include_bytes!("data/173.bin"),
		0x174 => include_bytes!("data/174.bin"),
		0x175 => include_bytes!("data/175.bin"),
		0x176 => include_bytes!("data/176.bin"),
		0x177 => include_bytes!("data/177.bin"),
		0x178 => include_bytes!("data/178.bin"),
		0x179 => include_bytes!("data/179.bin"),
		0x17a => include_bytes!("data/17a.bin"),
		0x17b => include_bytes!("data/17b.bin"),
		0x17c => include_bytes!("data/17c.bin"),
		0x17d => include_bytes!("data/17d.bin"),
		0x17e => include_bytes!("data/17e.bin"),
		0x17f => include_bytes!("data/17f.bin"),
		0x180 => include_bytes!("data/180.bin"),
		0x181 => include_bytes!("data/181.bin"),
		0x182 => include_bytes!("data/182.bin"),
		0x183 => include_bytes!("data/183.bin"),
		0x184 => include_bytes!("data/184.bin"),
		0x185 => include_bytes!("data/185.bin"),
		0x186 => include_bytes!("data/186.bin"),
		0x187 => include_bytes!("data/187.bin"),
		0x188 => include_bytes!("data/188.bin"),
		0x189 => include_bytes!("data/189.bin"),
		0x18a => include_bytes!("data/18a.bin"),
		0x18b => include_bytes!("data/18b.bin"),
		0x18c => include_bytes!("data/18c.bin"),
		0x18d => include_bytes!("data/18d.bin"),
		0x1b0 => include_bytes!("data/1b0.bin"),
		0x1b1 => include_bytes!("data/1b1.bin"),
		0x1b2 => include_bytes!("data/1b2.bin"),
		0x1bc => include_bytes!("data/1bc.bin"),
		0x1d0 => include_bytes!("data/1d0.bin"),
		0x1d1 => include_bytes!("data/1d1.bin"),
		0x1d2 => include_bytes!("data/1d2.bin"),
		0x1d3 => include_bytes!("data/1d3.bin"),
		0x1d4 => include_bytes!("data/1d4.bin"),
		0x1d5 => include_bytes!("data/1d5.bin"),
		0x1d6 => include_bytes!("data/1d6.bin"),
		0x1d7 => include_bytes!("data/1d7.bin"),
		0x1e0 => include_bytes!("data/1e0.bin"),
		0x1e1 => include_bytes!("data/1e1.bin"),
		0x1e2 => include_bytes!("data/1e2.bin"),
		0x1e8 => include_bytes!("data/1e8.bin"),
		0x1e9 => include_bytes!("data/1e9.bin"),
		0x1ec => include_bytes!("data/1ec.bin"),
		0x1ed => include_bytes!("data/1ed.bin"),
		0x1ee => include_bytes!("data/1ee.bin"),
		0x1f0 => include_bytes!("data/1f0.bin"),
		0x1f1 => include_bytes!("data/1f1.bin"),
		0x1f2 => include_bytes!("data/1f2.bin"),
		0x1f3 => include_bytes!("data/1f3.bin"),
		0x1f4 => include_bytes!("data/1f4.bin"),
		0x1f5 => include_bytes!("data/1f5.bin"),
		0x1f6 => include_bytes!("data/1f6.bin"),
		0x1f7 => include_bytes!("data/1f7.bin"),
		0x1f8 => include_bytes!("data/1f8.bin"),
		0x1f9 => include_bytes!("data/1f9.bin"),
		0x1fa => include_bytes!("data/1fa.bin"),
		0x1fb => include_bytes!("data/1fb.bin"),
		0x200 => include_bytes!("data/200.bin"),
		0x201 => include_bytes!("data/201.bin"),
		0x202 => include_bytes!("data/202.bin"),
		0x203 => include_bytes!("data/203.bin"),
		0x204 => include_bytes!("data/204.bin"),
		0x205 => include_bytes!("data/205.bin"),
		0x206 => include_bytes!("data/206.bin"),
		0x207 => include_bytes!("data/207.bin"),
		0x208 => include_bytes!("data/208.bin"),
		0x209 => include_bytes!("data/209.bin"),
		0x20a => include_bytes!("data/20a.bin"),
		0x20b => include_bytes!("data/20b.bin"),
		0x20c => include_bytes!("data/20c.bin"),
		0x20d => include_bytes!("data/20d.bin"),
		0x20e => include_bytes!("data/20e.bin"),
		0x20f => include_bytes!("data/20f.bin"),
		0x210 => include_bytes!("data/210.bin"),
		0x211 => include_bytes!("data/211.bin"),
		0x212 => include_bytes!("data/212.bin"),
		0x213 => include_bytes!("data/213.bin"),
		0x214 => include_bytes!("data/214.bin"),
		0x215 => include_bytes!("data/215.bin"),
		0x216 => include_bytes!("data/216.bin"),
		0x217 => include_bytes!("data/217.bin"),
		0x218 => include_bytes!("data/218.bin"),
		0x219 => include_bytes!("data/219.bin"),
		0x21a => include_bytes!("data/21a.bin"),
		0x21b => include_bytes!("data/21b.bin"),
		0x21c => include_bytes!("data/21c.bin"),
		0x21d => include_bytes!("data/21d.bin"),
		0x21e => include_bytes!("data/21e.bin"),
		0x21f => include_bytes!("data/21f.bin"),
		0x220 => include_bytes!("data/220.bin"),
		0x221 => include_bytes!("data/221.bin"),
		0x222 => include_bytes!("data/222.bin"),
		0x223 => include_bytes!("data/223.bin"),
		0x224 => include_bytes!("data/224.bin"),
		0x225 => include_bytes!("data/225.bin"),
		0x226 => include_bytes!("data/226.bin"),
		0x227 => include_bytes!("data/227.bin"),
		0x228 => include_bytes!("data/228.bin"),
		0x229 => include_bytes!("data/229.bin"),
		0x22a => include_bytes!("data/22a.bin"),
		0x22b => include_bytes!("data/22b.bin"),
		0x22c => include_bytes!("data/22c.bin"),
		0x22d => include_bytes!("data/22d.bin"),
		0x22e => include_bytes!("data/22e.bin"),
		0x22f => include_bytes!("data/22f.bin"),
		0x230 => include_bytes!("data/230.bin"),
		0x231 => include_bytes!("data/231.bin"),
		0x232 => include_bytes!("data/232.bin"),
		0x233 => include_bytes!("data/233.bin"),
		0x234 => include_bytes!("data/234.bin"),
		0x235 => include_bytes!("data/235.bin"),
		0x236 => include_bytes!("data/236.bin"),
		0x237 => include_bytes!("data/237.bin"),
		0x238 => include_bytes!("data/238.bin"),
		0x239 => include_bytes!("data/239.bin"),
		0x23a => include_bytes!("data/23a.bin"),
		0x23b => include_bytes!("data/23b.bin"),
		0x23c => include_bytes!("data/23c.bin"),
		0x23d => include_bytes!("data/23d.bin"),
		0x23e => include_bytes!("data/23e.bin"),
		0x23f => include_bytes!("data/23f.bin"),
		0x240 => include_bytes!("data/240.bin"),
		0x241 => include_bytes!("data/241.bin"),
		0x242 => include_bytes!("data/242.bin"),
		0x243 => include_bytes!("data/243.bin"),
		0x244 => include_bytes!("data/244.bin"),
		0x245 => include_bytes!("data/245.bin"),
		0x246 => include_bytes!("data/246.bin"),
		0x247 => include_bytes!("data/247.bin"),
		0x248 => include_bytes!("data/248.bin"),
		0x249 => include_bytes!("data/249.bin"),
		0x24a => include_bytes!("data/24a.bin"),
		0x24b => include_bytes!("data/24b.bin"),
		0x24c => include_bytes!("data/24c.bin"),
		0x24d => include_bytes!("data/24d.bin"),
		0x24e => include_bytes!("data/24e.bin"),
		0x24f => include_bytes!("data/24f.bin"),
		0x250 => include_bytes!("data/250.bin"),
		0x251 => include_bytes!("data/251.bin"),
		0x252 => include_bytes!("data/252.bin"),
		0x253 => include_bytes!("data/253.bin"),
		0x254 => include_bytes!("data/254.bin"),
		0x255 => include_bytes!("data/255.bin"),
		0x256 => include_bytes!("data/256.bin"),
		0x257 => include_bytes!("data/257.bin"),
		0x258 => include_bytes!("data/258.bin"),
		0x259 => include_bytes!("data/259.bin"),
		0x25a => include_bytes!("data/25a.bin"),
		0x25b => include_bytes!("data/25b.bin"),
		0x25c => include_bytes!("data/25c.bin"),
		0x25d => include_bytes!("data/25d.bin"),
		0x25e => include_bytes!("data/25e.bin"),
		0x25f => include_bytes!("data/25f.bin"),
		0x260 => include_bytes!("data/260.bin"),
		0x261 => include_bytes!("data/261.bin"),
		0x262 => include_bytes!("data/262.bin"),
		0x263 => include_bytes!("data/263.bin"),
		0x264 => include_bytes!("data/264.bin"),
		0x265 => include_bytes!("data/265.bin"),
		0x266 => include_bytes!("data/266.bin"),
		0x267 => include_bytes!("data/267.bin"),
		0x268 => include_bytes!("data/268.bin"),
		0x269 => include_bytes!("data/269.bin"),
		0x26a => include_bytes!("data/26a.bin"),
		0x26b => include_bytes!("data/26b.bin"),
		0x26c => include_bytes!("data/26c.bin"),
		0x26d => include_bytes!("data/26d.bin"),
		0x26e => include_bytes!("data/26e.bin"),
		0x26f => include_bytes!("data/26f.bin"),
		0x270 => include_bytes!("data/270.bin"),
		0x271 => include_bytes!("data/271.bin"),
		0x272 => include_bytes!("data/272.bin"),
		0x273 => include_bytes!("data/273.bin"),
		0x274 => include_bytes!("data/274.bin"),
		0x275 => include_bytes!("data/275.bin"),
		0x276 => include_bytes!("data/276.bin"),
		0x277 => include_bytes!("data/277.bin"),
		0x278 => include_bytes!("data/278.bin"),
		0x279 => include_bytes!("data/279.bin"),
		0x27a => include_bytes!("data/27a.bin"),
		0x27b => include_bytes!("data/27b.bin"),
		0x27c => include_bytes!("data/27c.bin"),
		0x27d => include_bytes!("data/27d.bin"),
		0x27e => include_bytes!("data/27e.bin"),
		0x27f => include_bytes!("data/27f.bin"),
		0x280 => include_bytes!("data/280.bin"),
		0x281 => include_bytes!("data/281.bin"),
		0x282 => include_bytes!("data/282.bin"),
		0x283 => include_bytes!("data/283.bin"),
		0x284 => include_bytes!("data/284.bin"),
		0x285 => include_bytes!("data/285.bin"),
		0x286 => include_bytes!("data/286.bin"),
		0x287 => include_bytes!("data/287.bin"),
		0x288 => include_bytes!("data/288.bin"),
		0x289 => include_bytes!("data/289.bin"),
		0x28a => include_bytes!("data/28a.bin"),
		0x28b => include_bytes!("data/28b.bin"),
		0x28c => include_bytes!("data/28c.bin"),
		0x28d => include_bytes!("data/28d.bin"),
		0x28e => include_bytes!("data/28e.bin"),
		0x28f => include_bytes!("data/28f.bin"),
		0x290 => include_bytes!("data/290.bin"),
		0x291 => include_bytes!("data/291.bin"),
		0x292 => include_bytes!("data/292.bin"),
		0x293 => include_bytes!("data/293.bin"),
		0x294 => include_bytes!("data/294.bin"),
		0x295 => include_bytes!("data/295.bin"),
		0x296 => include_bytes!("data/296.bin"),
		0x297 => include_bytes!("data/297.bin"),
		0x298 => include_bytes!("data/298.bin"),
		0x299 => include_bytes!("data/299.bin"),
		0x29a => include_bytes!("data/29a.bin"),
		0x29b => include_bytes!("data/29b.bin"),
		0x29c => include_bytes!("data/29c.bin"),
		0x29d => include_bytes!("data/29d.bin"),
		0x29e => include_bytes!("data/29e.bin"),
		0x29f => include_bytes!("data/29f.bin"),
		0x2a0 => include_bytes!("data/2a0.bin"),
		0x2a1 => include_bytes!("data/2a1.bin"),
		0x2a2 => include_bytes!("data/2a2.bin"),
		0x2a3 => include_bytes!("data/2a3.bin"),
		0x2a4 => include_bytes!("data/2a4.bin"),
		0x2a5 => include_bytes!("data/2a5.bin"),
		0x2a6 => include_bytes!("data/2a6.bin"),
		0x2a7 => include_bytes!("data/2a7.bin"),
		0x2a8 => include_bytes!("data/2a8.bin"),
		0x2a9 => include_bytes!("data/2a9.bin"),
		0x2aa => include_bytes!("data/2aa.bin"),
		0x2ab => include_bytes!("data/2ab.bin"),
		0x2ac => include_bytes!("data/2ac.bin"),
		0x2ad => include_bytes!("data/2ad.bin"),
		0x2ae => include_bytes!("data/2ae.bin"),
		0x2af => include_bytes!("data/2af.bin"),
		0x2b0 => include_bytes!("data/2b0.bin"),
		0x2b1 => include_bytes!("data/2b1.bin"),
		0x2b2 => include_bytes!("data/2b2.bin"),
		0x2b3 => include_bytes!("data/2b3.bin"),
		0x2b4 => include_bytes!("data/2b4.bin"),
		0x2b5 => include_bytes!("data/2b5.bin"),
		0x2b6 => include_bytes!("data/2b6.bin"),
		0x2b7 => include_bytes!("data/2b7.bin"),
		0x2b8 => include_bytes!("data/2b8.bin"),
		0x2b9 => include_bytes!("data/2b9.bin"),
		0x2ba => include_bytes!("data/2ba.bin"),
		0x2bb => include_bytes!("data/2bb.bin"),
		0x2bc => include_bytes!("data/2bc.bin"),
		0x2bd => include_bytes!("data/2bd.bin"),
		0x2be => include_bytes!("data/2be.bin"),
		0x2bf => include_bytes!("data/2bf.bin"),
		0x2c0 => include_bytes!("data/2c0.bin"),
		0x2c1 => include_bytes!("data/2c1.bin"),
		0x2c2 => include_bytes!("data/2c2.bin"),
		0x2c3 => include_bytes!("data/2c3.bin"),
		0x2c4 => include_bytes!("data/2c4.bin"),
		0x2c5 => include_bytes!("data/2c5.bin"),
		0x2c6 => include_bytes!("data/2c6.bin"),
		0x2c7 => include_bytes!("data/2c7.bin"),
		0x2c8 => include_bytes!("data/2c8.bin"),
		0x2c9 => include_bytes!("data/2c9.bin"),
		0x2ca => include_bytes!("data/2ca.bin"),
		0x2cb => include_bytes!("data/2cb.bin"),
		0x2cc => include_bytes!("data/2cc.bin"),
		0x2cd => include_bytes!("data/2cd.bin"),
		0x2ce => include_bytes!("data/2ce.bin"),
		0x2d0 => include_bytes!("data/2d0.bin"),
		0x2d1 => include_bytes!("data/2d1.bin"),
		0x2d2 => include_bytes!("data/2d2.bin"),
		0x2d3 => include_bytes!("data/2d3.bin"),
		0x2d4 => include_bytes!("data/2d4.bin"),
		0x2d5 => include_bytes!("data/2d5.bin"),
		0x2d6 => include_bytes!("data/2d6.bin"),
		0x2d7 => include_bytes!("data/2d7.bin"),
		0x2d8 => include_bytes!("data/2d8.bin"),
		0x2d9 => include_bytes!("data/2d9.bin"),
		0x2da => include_bytes!("data/2da.bin"),
		0x2db => include_bytes!("data/2db.bin"),
		0x2dc => include_bytes!("data/2dc.bin"),
		0x2dd => include_bytes!("data/2dd.bin"),
		0x2de => include_bytes!("data/2de.bin"),
		0x2e0 => include_bytes!("data/2e0.bin"),
		0x2e1 => include_bytes!("data/2e1.bin"),
		0x2e2 => include_bytes!("data/2e2.bin"),
		0x2e3 => include_bytes!("data/2e3.bin"),
		0x2e4 => include_bytes!("data/2e4.bin"),
		0x2e5 => include_bytes!("data/2e5.bin"),
		0x2e6 => include_bytes!("data/2e6.bin"),
		0x2e7 => include_bytes!("data/2e7.bin"),
		0x2e8 => include_bytes!("data/2e8.bin"),
		0x2e9 => include_bytes!("data/2e9.bin"),
		0x2ea => include_bytes!("data/2ea.bin"),
		0x2eb => include_bytes!("data/2eb.bin"),
		0x2f8 => include_bytes!("data/2f8.bin"),
		0x2f9 => include_bytes!("data/2f9.bin"),
		0x2fa => include_bytes!("data/2fa.bin"),
		0x300 => include_bytes!("data/300.bin"),
		0x301 => include_bytes!("data/301.bin"),
		0x302 => include_bytes!("data/302.bin"),
		0x303 => include_bytes!("data/303.bin"),
		0x304 => include_bytes!("data/304.bin"),
		0x305 => include_bytes!("data/305.bin"),
		0x306 => include_bytes!("data/306.bin"),
		0x307 => include_bytes!("data/307.bin"),
		0x308 => include_bytes!("data/308.bin"),
		0x309 => include_bytes!("data/309.bin"),
		0x30a => include_bytes!("data/30a.bin"),
		0x30b => include_bytes!("data/30b.bin"),
		0x30c => include_bytes!("data/30c.bin"),
		0x30d => include_bytes!("data/30d.bin"),
		0x30e => include_bytes!("data/30e.bin"),
		0x30f => include_bytes!("data/30f.bin"),
		0x310 => include_bytes!("data/310.bin"),
		0x311 => include_bytes!("data/311.bin"),
		0x312 => include_bytes!("data/312.bin"),
		0x313 => include_bytes!("data/313.bin"),
		0xe00 => include_bytes!("data/e00.bin"),
		_ => &[]
	}
}