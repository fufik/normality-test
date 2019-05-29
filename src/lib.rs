#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static!{
    static ref CHI2_CRIT_VAL: HashMap<&'static str,[f32;30]> = {
        let mut m = HashMap::new();
        m.insert("0.01",[6.6,9.2,11.3,13.3,15.1,16.8,18.5,20.1,21.7,23.2,24.7,26.2,27.7,29.1,30.6,32.0,33.4,34.8,36.2,37.6,38.9,40.3,41.6,43.0,44.3,45.6,47.0,48.3,49.6,50.9]);
        m.insert("0.025",[5.0,7.4,9.4,11.1,12.8,14.4,16.0,17.5,19.0,20.5,21.9,23.3,24.7,26.1,27.5,28.8,30.2,31.5,32.9,34.2,35.5,36.8,38.1,39.4,40.6,41.9,43.2,44.5,45.7,47.0]);
        m.insert("0.05",[3.8,6.0,7.8,9.5,11.1,12.6,14.1,15.5,16.9,18.3,19.7,21.0,22.4,23.7,25.0,26.3,27.6,28.9,30.1,31.4,32.7,33.9,35.2,36.4,37.7,38.9,40.1,41.3,42.6,43.8]);
        //m.insert("0.95",);
        //m.insert("0.975",);
        //m.insert("0.99",);
        m
    };
    static ref LAPLACE_VAL: ([f32;201],[f32;57]) = {
        let m :[f32;201]= [0.0000,0.0040,0.0080,0.0120,0.0160,0.0199,0.0239,0.0279,0.0319,0.0359,0.039,0.0438,0.0478,0.0517,0.0557,0.0596,0.0636,0.0675,0.071,0.0753,0.0793,0.0832,0.0871,0.0910,0.0948,0.0987,0.1026,0.1064,0.1103,0.1141,0.1179,0.1217,0.1255,0.1293,0.1331,0.1368,0.1406,0.1443,0.1480,0.1517,0.1554,0.1591,0.1628,0.1664,0.1700,0.1736,0.1772,0.1808,0.1844,0.1879,0.1915,0.1950,0.1985,0.2019,0.2054,0.2088,0.2123,0.2157,0.2190,0.2224,0.2257,0.2291,0.2324,0.2357,0.2389,0.2422,0.2454,0.2486,0.2517,0.2549,0.2580,0.2611,0.2642,0.2673,0.2703,0.2734,0.2764,0.2794,0.2823,0.2852,0.2881,0.2910,0.2939,0.2967,0.2995,0.3023,0.3051,0.3078,0.3106,0.3133,0.3159,0.3186,0.3212,0.3238,0.3264,0.3289,0.3315,0.3340,0.3365,0.3389,0.3413,0.3438,0.3461,0.3485,0.3508,0.3531,0.3554,0.3577,0.3599,0.3621,0.3643,0.3665,0.3686,0.3708,0.3729,0.3749,0.3770,0.3790,0.3810,0.3830,0.3849,0.3869,0.3883,0.3907,0.3925,0.3944,0.3962,0.3980,0.3997,0.4015,0.4032,0.4049,0.4066,0.4082,0.4099,0.4115,0.4131,0.4147,0.4162,0.4177,0.4192,0.4207,0.4222,0.4236,0.4251,0.4265,0.4279,0.4292,0.4306,0.4319,0.4332,0.4345,0.4357,0.4370,0.4382,0.4394,0.4406,0.4418,0.4429,0.4441,0.4452,0.4463,0.4474,0.4484,0.4495,0.4505,0.4515,0.4525,0.4535,0.4545,0.4554,0.4564,0.4573,0.4582,0.4591,0.4599,0.4608,0.4616,0.4625,0.4633,0.4641,0.4649,0.4656,0.4664,0.4671,0.4678,0.4686,0.4693,0.4699,0.4706,0.4713,0.4719,0.4726,0.4732,0.4738,0.4744,0.4750,0.4756,0.4761,0.4767,0.4772];
        let b:[f32;57] = [0.4783,0.4793,0.4803,0.4812,0.4821,0.4830,0.4838,0.4846,0.4854,0.4861,0.4868,0.4875,0.4881,0.4887,0.4893,0.4898,0.4904,0.4909,0.4913,0.4918,0.4922,0.4927,0.4931,0.4934,0.4938,0.4941,0.4945,0.4948,0.4951,0.4953,0.4956,0.4959,0.4961,0.4963,0.4965,0.4967,0.4969,0.4971,0.4973,0.4974,0.4976,0.4977,0.4979,0.4980,0.4981,0.4982,0.4984,0.4985,0.4986,0.49865,0.49931,0.49966,0.499841,0.499928,0.499968,0.499997,0.499997];
        (m,b)
    };
}

pub fn get_chi2_crit(alpha: f32,df: usize) -> Result<f32,&'static str> {
    if df > 30{
        return Err("No table availabe with given degrees of freedom.")
    }
    if CHI2_CRIT_VAL.contains_key(alpha.to_string().as_str()){
        return Ok(CHI2_CRIT_VAL.get(alpha.to_string().as_str()).unwrap()[df-1])
    }
    else {
        return Err("No table available for given alpha.")
    }
}

pub fn get_laplace_error(x:f32)-> Result<f32,&'static str> {
    if x>5.0 {
        return Ok(0.5)
    }
    if x>2.0{
        let x = x - 2.0;
        let mut y: u32 = (x * 100.0).round() as u32;
        if y%2!=0{
            y = y + 1;
        }
        return Ok(*LAPLACE_VAL.1.get(y as usize).unwrap())
    }
    else{
        return Ok(*LAPLACE_VAL.0.get((x * 100.0).round() as usize).unwrap())
    } 
}

pub mod dstrb_thr_freq{
    pub fn normal(gen_sample: Vec<u32>,emp_freq: Vec<u32>) -> Vec<f32>{
        //1.1Dividing into the intervals
        let mut m: Vec<u32> = Vec::new();
        let min = *gen_sample.iter().min().unwrap();
        let max = *gen_sample.iter().max().unwrap();
        let ran: Vec<u32> = (min..=max).map(|f|f).collect();
        let ssize = (gen_sample.len()/emp_freq.len()) as usize;
        for mut i in (&ran).iter().step_by(ssize){
            m.push(*i);
        }
        if m.len() < emp_freq.len()+1{
            m.push(*ran.last().unwrap());
        }
        let mut mt: Vec<(u32,u32)> = Vec::new();
        let mut i = 1;
        while i < m.len(){
            mt.push((m[i-1],m[i]));
            i = i + 1;
        }
        //1.2Finding centres
        let mut m_cen : Vec<f32> = Vec::new();
        for i in mt.iter(){
            m_cen.push((i.0 as f32 + i.1 as f32)/2.0);
        }
        //2


        return Vec::new()
    }
}

pub mod prove{       
    pub fn pearson_chi2_normal(gen_sample: Vec<u32>,emp_freqs: Vec<u32>,alpha: f32){
        //df=s-1-r
        //Normal Distribution has two degrees of freedom, hence r=2
        //s is amount of intervals
        let df = emp_freqs.len()-1-2;
            
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn access_chi2_val() {
        assert_eq!(get_chi2_crit(0.01,3), Ok(11.3));
    }
    #[test]
    fn access_laplace_error_val() {
        assert_eq!(get_laplace_error(0.03), Ok(0.0120));
    }
    #[test]
    fn access_laplace_error_limit() {
        assert_eq!(get_laplace_error(5.03), Ok(0.5));
    }


}
