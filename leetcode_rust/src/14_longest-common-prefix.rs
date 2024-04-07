#[allow(dead_code)]

pub struct Solution {}



impl Solution {
    pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
        strs.iter()
            .max()  //找到最长的那个串
            .unwrap()   //拿到上面max的结果
            .chars()    //拿到当前迭代的char
            .zip(strs.iter().min().unwrap().chars())    //和min拿到最短串的char打包成p[a,b]
            .take_while(|x| x.0 == x.1) //当a==b时取值
            .map(|x| x.0)   //将[a,b]替换为a
            .collect()  //收集上述规则迭代器的结果
    }

}

//作者：独坐山亭√
//链接：https://leetcode.cn/problems/longest-common-prefix/solutions/1177899/rust-zui-chang-gong-gong-qian-zhui-ti-ji-dwuj/
//来源：力扣（LeetCode）
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。