// brings deliver_order into scope
use crate::deliver_order;
pub fn take_order() {}
pub fn serve_order() {
    deliver_order();
}
pub fn take_payment() {}
