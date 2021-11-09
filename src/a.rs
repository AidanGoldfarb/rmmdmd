use crate::util::*;

/*
    (29)
*/
#[allow(unused)]
pub fn atn(n: usize) -> f64 {
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
    let mut t: f64 = atn(n as usize); //0.0;
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
    let (mut c, upper_bound) = (0.0, n.log2());
    t1 += (2.0_f64.powf(c).powf(2.0))
        * I((j - 1.0 % 2.0 * n) % 2.0_f64.powf(n + 1.0) + 1.0 > 2.0_f64.powf(n));
    while c <= upper_bound {
        t1 += (2.0_f64.powf(c).powf(2.0))
            * I((j - 1.0 % 2.0 * n) % 2.0_f64.powf(c + 1.0) + 1.0 > 2.0_f64.powf(c));
        t2 += (2.0_f64.powf(c).powf(2.0))
            * I((i - 1.0 % 2.0 * n) % 2.0_f64.powf(c + 1.0) + 1.0 > 2.0_f64.powf(c));
        c += 1.0;
    }
    t1 + t2
}

/*
    (23)
*/
#[allow(unused, non_snake_case)]
pub fn F_A(i: usize, j: usize, n: usize) -> f64 {
    f_T(i, j, n) + f_AB(i as f64, j as f64, n as f64)
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
                assert_eq!(t_N_a(i,j,1), *c)
            }
        }
    }

    #[test]
    fn verify_30_sz_2(){
        let bt2 = str_to_vec("26 27 30 31
        28 29 32 33");
        for (i,r) in bt2.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i,j,2), *c)
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
                assert_eq!(t_N_a(i,j,4), *c)
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
        1776 1777 1780 1781 1792 1973 1796 1797 1840 1841 1844 1845 1856 1857 1860 1861");
        for (i,r) in bt8.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N_a(i,j,8), *c)
            }
        }
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

}
