mod pb;

// 修改导入路径以匹配生成的模块结构
use crate::pb::common::Money;
use crate::pb::payment::Address as PaymentAddress;
use crate::pb::user::Address as UserAddress;

fn process_order(
    user_id: &str,
    shipping_address: UserAddress,
    billing_address: PaymentAddress,
    amount: Money,
) {
    // 使用不同的Address类型
    println!(
        "Shipping to: {}, {}",
        shipping_address.street, shipping_address.city
    );
    println!("Billing to: {}", billing_address.full_address);
}

fn main() {
    // 创建一个用户地址
    let shipping_address = UserAddress {
        street: "中关村大街1号".to_string(),
        city: "北京".to_string(),
        state: "北京市".to_string(),
        country: "中国".to_string(),
        postal_code: "100080".to_string(),
    };

    // 创建一个支付地址
    let billing_address = PaymentAddress {
        full_address: "中国北京市海淀区中关村大街1号 100080".to_string(),
        recipient_name: "abce".to_string(),
        phone_number: "12345678901".to_string(),
    };

    // 创建金额
    let amount = Money {
        currency_code: "CNY".to_string(),
        amount: 100_500_000_000, // 100.5元
    };

    // 处理订单
    process_order("user123", shipping_address, billing_address, amount);

    println!("订单处理完成!");
}
