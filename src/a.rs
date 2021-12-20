use crate::util::*;

/*
    (29)
*/
#[allow(unused)]
pub fn atn(n: usize) -> f64 {
    if n == 1{
        return 3.0;
    }
    let n = n as f64;

    let t1 = T(n);
    let t2 = 2.0 * n * n;
    let t3 = 8.0 * T(n / 2.0);
    let mut t4 = 0.0;
    let mut count = n;
    let mut i = 1.0;
    while count > 1.0 {
        t4 += 2.0 * T(2.0_f64.powf(i - 1.0));
        count /= 2.0;
        i += 1.0;
    }
    t1 + t2 + t3 - t4
}

/*
    (algo 1)
*/
#[allow(unused)]
pub fn algo1(i: usize, j: usize, n: usize) -> f64 {
    let mut i = i as f64;
    let mut j = j as f64;
    let n = n as f64;
    let mut t: f64 = 0.0;//atn(n as usize); //0.0;
    let mut n_temp: f64 = n as f64;

    if j > n {
        t = n * n;
    }
    while n_temp > 1.0 {
        if j > n_temp {
            t += n * n;
        }
        if i > n_temp {
            t += 2.0 * (n / 2.0) * (n / 2.0)
        }
        j = ((j - 1.0) % n_temp) + 1.0;
        i = ((i - 1.0) % n_temp) + 1.0;
        n_temp /= 2.0;
    }
    t
}

/*
    (30)   
*/
#[allow(unused, non_snake_case)]
pub fn t_N_a(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;
    
    let (mut t1, mut t2) = (0.0, 0.0);
    let (mut k, upper_bound) = (0.0, n.log2());
    // t1 += (2.0_f64.powf(k).powf(2.0))
    //     * I(((j - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(0.0));
    while k <= upper_bound {
        //println!("i:{} j:{} k:{} IND: {}", i,j,k,I((((j - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0) > 2.0_f64.powf(k)));
        t1 += (2.0_f64.powf(k).powf(2.0))
            * I((((j - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0) > 2.0_f64.powf(k));
        
        t2 += (2.0_f64.powf(k).powf(2.0))
            * I((((i - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0) > 2.0_f64.powf(k));
        k += 1.0;
    }
    t1 + 2.0*t2
}

/*
    (23)
*/
#[allow(unused, non_snake_case)]
pub fn F_A(i: usize, j: usize, n: usize) -> f64 {
    //println!("f_T: {}\nf_AB: {}", f_T(i, j, n), f_AB(i as f64, (j as f64) % n as f64, n as f64));
    f_T(i, j, n) + f_AB(i as f64, (j as f64) % n as f64, n as f64)
}

/*
    (31)
*/
#[allow(unused, non_snake_case)]
pub fn f_T(i: usize, j: usize, n: usize) -> f64 {
    atn(n) + t_N_a(i, j, n)
}

/*
    (41)
*/
#[allow(unused, non_snake_case)]
pub fn f_AB(i: f64, j: f64, n: f64) -> f64 {
    
    if n < 1.0 {
        return 0.0;
    }
    if n == 1.0 {
        return 4.0;
    }

    let t1 = 4.0*n*n * (n/2.0).powf(2.0) * I((n/4.0 < i && i <= n/2.0) && j <= n/2.0);
    let t2 =
        2.0 * (n / 2.0).powf(2.0) * I((n / 4.0 < i && i <= n / 2.0) && (n / 2.0 < j && j <= n));
    let t3 =
        (n / 2.0).powf(2.0) * I((n / 2.0 < i && i <= 3.0 * n / 4.0) && (n / 2.0 < j && j <= n));
    let t4 = 2.0 * (n / 2.0).powf(2.0) * I((n / 2.0 < i && i <= 3.0 * n / 4.0) && (j <= n / 2.0));
    let t5 = ((n / 2.0).powf(2.0) + f_AB(i - n / 2.0, j, n / 2.0))
        * I(i > 3.0 * n / 4.0 && j <= n / 2.0);
    let t6 = f_AB(i - n / 2.0, j - n / 2.0, n / 2.0) * I(i > 3.0 * n / 4.0 && j > n / 2.0);
    let t7 = ((n / 2.0).powf(2.0) + f_AB(i, j - n / 2.0, n / 2.0)) * I(i < n / 4.0 && j > n / 2.0);
    let t8 = f_AB(i, j, n / 2.0) * I(i < n / 4.0 && j <= n / 2.0);

    t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
}

/*
*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq};
    use crate::util::str_to_vec;
    
    #[test]
    //atn (29) //Tested from mats (25)[0][0] .. (28)[0][0]
    fn verify_29(){
        let gt = vec![3.0,26.0,214.0,1734.0];
        let depth = vec![1,2,4,8];
        for (t,n) in gt.iter().zip(depth.iter()){
            assert_eq!(atn(*n), *t);
        }
    }

    #[test]
    fn verify_30_sz_1(){
        let bt1 = str_to_vec("3 4");
        for (i,r) in bt1.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i+1,j+1,1), *c-atn(1),"failed on [{}][{}]", i,j)
            }
        }
    }
    // true, true, true, true, true, true, true, true]
    // [true, true, true, true, true, true, true, true]
    // [true, true, true, true, true, true, true, true]
    // [true, true, true, true, true, true, true, tru
    #[test]
    fn verify_30_sz_2(){
        let bt2 = str_to_vec("26 27 30 31
        28 29 32 33");
        for (i,r) in bt2.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i+1,j+1,2), *c-atn(2),"failed on [{}][{}]", i,j)
            }
        }
    }

    #[test]
    fn verify_30_sz_4(){
        let bt4 = str_to_vec("214 215 218 219 230 231 234 235
        216 217 220 221 232 233 236 237
        222 223 226 227 238 239 242 243
        224 225 228 229 240 241 244 245");
        for (i,r) in bt4.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i+1,j+1,4), *c-atn(4),"failed on [{}][{}]", i,j)
            }
        }
    }

    #[test]
    fn verify_30_sz_8(){
        let bt8 = str_to_vec("1734 1735 1738 1739 1750 1751 1754 1755 1798 1799 1802 1803 1814 1815 1818 1819
        1736 1737 1740 1741 1752 1753 1756 1757 1800 1801 1804 1805 1816 1817 1820 1821
        1742 1743 1746 1747 1758 1759 1762 1763 1806 1807 1810 1811 1822 1823 1826 1827
        1744 1745 1748 1749 1760 1761 1764 1765 1808 1809 1812 1813 1824 1825 1828 1829
        1766 1767 1770 1771 1782 1783 1786 1787 1830 1831 1834 1835 1846 1847 1850 1851
        1768 1769 1772 1773 1784 1785 1788 1789 1832 1833 1836 1837 1848 1849 1852 1853
        1774 1775 1778 1779 1790 1791 1794 1795 1838 1839 1842 1843 1854 1855 1858 1859
        1776 1777 1780 1781 1792 1793 1796 1797 1840 1841 1844 1845 1856 1857 1860 1861");
        for (i,r) in bt8.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i+1,j+1,8), *c - atn(8),"failed on [{}][{}]", i,j)
            }
        }
    }

    #[test]
    #[ignore]
    fn verify_30_sz_8_compl(){ 
        let bt8 = str_to_vec("1734 1735 1738 1739 1750 1751 1754 1755 1798 1799 1802 1803 1814 1815 1818 1819
     1736 1737 1740 1741 1752 1753 1756 1757 1800 1801 1804 1805 1816 1817 1820 1821
     1742 1743 1746 1747 1758 1759 1762 1763 1806 1807 1810 1811 1822 1823 1826 1827
     1744 1745 1748 1749 1760 1761 1764 1765 1808 1809 1812 1813 1824 1825 1828 1829
     1766 1767 1770 1771 1782 1783 1786 1787 1830 1831 1834 1835 1846 1847 1850 1851
     1768 1769 1772 1773 1784 1785 1788 1789 1832 1833 1836 1837 1848 1849 1852 1853
     1774 1775 1778 1779 1790 1791 1794 1795 1838 1839 1842 1843 1854 1855 1858 1859
     1776 1777 1780 1781 1792 1793 1796 1797 1840 1841 1844 1845 1856 1857 1860 1861");
        let mut correctness = vec![vec![false;16];8];
        let mut values = vec![vec![0.0;16];8];
        for (i,r) in bt8.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                values[i][j] = t_N_a(i+1,j+1,8);
                if t_N_a(i+1,j+1,8) == c - atn(8){
                    correctness[i][j] = true;
                }
            }
        }
        for r in correctness{
            println!("{:?}", r);
        }
        assert!(false);
    }

    #[test]
    fn verify_30_sz_4_compl(){ 
        let bt4 = str_to_vec("214 215 218 219 230 231 234 235
        216 217 220 221 232 233 236 237
        222 223 226 227 238 239 242 243
        224 225 228 229 240 241 244 245");
        //let mut correctness = vec![vec![false;8];4];
        for (i,r) in bt4.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i+1,j+1,4), c - atn(4))
                // if t_N_a(i+1,j+1,4) == c - atn(4){
                //     correctness[i][j] = true;
                // }
            }
        }
        // for r in correctness{
        //     println!("{:?}", r);
        // }
        // assert!(false);
    }


    #[test]
    fn verify_table() {
        let n = vec![1.0, 2.0, 4.0, 8.0];
        let tn_correct = vec![1.0, 12.0, 112.0, 960.0];
        let dn_correct = vec![3.0, 20.0, 144.0, 1088.0];
        for (i, val) in n.iter().enumerate() {
            assert_eq!(T(*val), tn_correct[i]);
            assert_eq!(D(*val), dn_correct[i]);
        }
    }

    #[test]
    #[ignore]
    fn verify_30_gt(){ 
        let sz = 8;
        let bt = str_to_vec("1734 1735 1738 1739 1750 1751 1754 1755 1798 1799 1802 1803 1814 1815 1818 1819
     1736 1737 1740 1741 1752 1753 1756 1757 1800 1801 1804 1805 1816 1817 1820 1821
     1742 1743 1746 1747 1758 1759 1762 1763 1806 1807 1810 1811 1822 1823 1826 1827
     1744 1745 1748 1749 1760 1761 1764 1765 1808 1809 1812 1813 1824 1825 1828 1829
     1766 1767 1770 1771 1782 1783 1786 1787 1830 1831 1834 1835 1846 1847 1850 1851
     1768 1769 1772 1773 1784 1785 1788 1789 1832 1833 1836 1837 1848 1849 1852 1853
     1774 1775 1778 1779 1790 1791 1794 1795 1838 1839 1842 1843 1854 1855 1858 1859
     1776 1777 1780 1781 1792 1973 1796 1797 1840 1841 1844 1845 1856 1857 1860 1861");
        let mut correctness = vec![vec![false;2*sz];sz];
        let mut values = vec![vec![0.0;2*sz];sz];
        for (i,r) in bt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                values[i][j] = c - atn(sz);//algo1(i+1,j+1,8);
                if algo1(i+1,j+1,sz) == c - atn(sz){
                    correctness[i][j] = true;
                }
            }
        }
        for r in values{
            println!("{:?}", r);
        }
        assert!(false);
    }

    #[test]
    fn verify_30_sz_32(){ 
        let sz = 16;
        let bt = str_to_vec("13958 13959 13962 13963 13974 13975 13978 13979 14022 14023 14026 14027 14038 14039 14042 14043 
            13960 13961 13964 13965 13976 13977 13980 13981 14024 14025 14028 14029 14040 14041 14044 14045 
            13966 13967 13970 13971 13982 13983 13986 13987 14030 14031 14034 14035 14046 14047 14050 14051
            13968 13969 13972 13973 13984 13985 13988 13989 14032 14033 14036 14037 14048 14049 14052 14053 
            13990 13991 13994 13995 14006 14007 14010 14011 14054 14055 14058 14059 14070 14071 14074 14075 
            13992 13993 13996 13997 14008 14009 14012 14013 14056 14057 14060 14061 14072 14073 14076 14077 
            13998 13999 14002 14003 14014 14015 14018 14019 14062 14063 14066 14067 14078 14079 14082 14083 
            14000 14001 14004 14005 14016 14017 14020 14021 14064 14065 14068 14069 14080 14081 14084 14085
            14086 14087 14090 14091 14102 14103 14106 14107 14150 14151 14154 14155 14166 14167 14170 14171  
            14088 14089 14092 14093 14104 14105 14108 14109 14152 14153 14156 14157 14168 14169 14172 14173
            14094 14095 14098 14099 14110 14111 14114 14115 14158 14159 14162 14163 14174 14175 14178 14179
            14096 14097 14100 14101 14112 14113 14116 14117 14160 14161 14164 14165 14176 14177 14180 14181
            14118 14119 14122 14123 14134 14135 14138 14139 14182 14183 14186 14187 14198 14199 14202 14203
            14120 14121 14124 14125 14136 14137 14140 14141 14184 14185 14188 14189 14200 14201 14204 14205
            14126 14127 14130 14131 14142 14143 14146 14147 14190 14191 14194 14195 14206 14207 14210 14211 
            14128 14129 14132 14133 14144 14145 14148 14149 14192 14193 14196 14197 14208 14209 14212 14213");


        // let mut correctness = vec![vec![false;2*sz];sz];
        // let mut values = vec![vec![0.0;2*sz];sz];
        for (i,r) in bt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                //values[i][j] = c - atn(sz);//algo1(i+1,j+1,8);
                assert_eq!(t_N_a(i+1,j+1,sz), c-atn(sz));
                // if t_N_a(i+1,j+1,sz) == c - atn(sz){
                //     correctness[i][j] = true;
                // }
            }
        }
        // for r in values{
        //     println!("{:?}", r);
        // }
        // assert!(false);
    }

    #[test]
    fn verify_41_sz_1(){
        let gt = vec![4.0];
        assert_eq!(f_AB(0.0,0.0,1.0), gt[0]);
    }

    #[test]
    fn verify_41_sz_2(){
        let gt = str_to_vec("16 17
        18 17");
        let sz = 2;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(f_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    #[test]
    fn verify_41_sz_4(){
        let gt = str_to_vec("64 65 68 69
        68 68 72 72
        72 72 68 68
        70 69 66 65");
        let sz = 4;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(f_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    #[test]
    fn verify_41_sz_8(){
        let gt = str_to_vec("256 257 260 261 272 273 276 277
        260 260 264 264 276 276 280 280
        272 272 272 272 288 288 288 288
        272 272 272 272 288 288 288 288
        288 288 288 288 272 272 272 272
        288 288 288 288 272 272 272 272
        280 280 276 276 264 264 260 260
        278 277 274 273 262 261 258 257");
        let sz = 8;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(f_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    #[test]
    fn verify_41_sz_8_compl(){
        let gt = str_to_vec("256 257 260 261 272 273 276 277
        260 260 264 264 276 276 280 280
        272 272 272 272 288 288 288 288
        272 272 272 272 288 288 288 288
        288 288 288 288 272 272 272 272
        288 288 288 288 272 272 272 272
        280 280 276 276 264 264 260 260
        278 277 274 273 262 261 258 257");
        let sz = 8;
        let mut correct = vec![vec![false;8];8];
        let mut values = vec![vec![0.0;8];8];
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                values[i][j] = f_AB((i+1) as f64,(j+1) as f64,sz as f64);
                if f_AB((i+1) as f64,(j+1) as f64,sz as f64) == *c{
                    correct[i][j] = true;
                }
                //assert_eq!(f_AB((i+1) as f64,(j+1) as f64,sz as f64), gt[i][j]);
            }
        }
        for r in values{
            println!("{:?}", r);
        }
        assert!(false);
    }
}
