/*
 * @Author: Lin Bowen
 * @Date: 2021-09-27 01:08:53
 * @LastEditTime: 2021-09-27 01:15:44
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \rust\enumeration\src\main.rs
 */
fn main() {
    {
        #[derive(Debug)]
        enum IpAddrKind{
            v4,
            v6
        }

        let four = IpAddrKind::v4;
        let six = IpAddrKind::v6;

        fn route(ip_type: IpAddrKind){}

        route(four);
        route(six);

        //用在结构体中
        #[derive(Debug)]
        struct IpAddr{
            kind:IpAddrKind,
            address:String
        }
        let home = IpAddr{
            kind:IpAddrKind::v4,
            address:String::from("127.0.0.1")
        };
        let loopback = IpAddr{
            kind:IpAddrKind::v6,
            address:String::from("::1")
        };
        println!("home:{:?}",home);
        println!("loopback:{:?}",loopback);

    }
}
