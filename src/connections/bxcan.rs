use bxcan;
use bxcan::Id;
use bxcan::{Frame, StandardId};
use core::convert::TryInto;
use core::future::Future;
use nb::block;

pub struct BxCanInterface<T: bxcan::Instance> {
    can: bxcan::Can<T>,
}

impl<T: bxcan::Instance> BxCanInterface<T> {
    pub fn new(can: bxcan::Can<T>) -> Self {
        Self { can: can }
    }
}

impl<T: bxcan::Instance> crate::TMCLConnnection for BxCanInterface<T> {
    type SendFuture<'a> = impl Future<Output = ()>;
    type ReceiveFuture<'a> = impl Future<Output = [u8; 8]>;

    // Send the bytearray [data] representing a TMCL command. The length of
    // [data] is 9. The hostID and module_id parameters may be used for extended
    // addressing options available on the implemented communication interface.
    fn _send<'a>(&mut self, host_id: u16, module_id: u16, data: [u8; 8]) -> Self::SendFuture<'a> {
        let frame_tx = Frame::new_data(
            StandardId::new(module_id).unwrap(),
            [
                data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ],
        );

        block!(self.can.transmit(&frame_tx)).unwrap();

        async move {}
    }
    //
    // Receive a TMCL reply and return it as a bytearray. The length of the
    // returned byte array is 9. The hostID and module_id parameters may be used
    // for extended addressing options available on the implemented
    // communication interface.
    fn _recv<'a>(&mut self, host_id: u16, module_id: u16) -> Self::ReceiveFuture<'a> {
        let frame_rx = block!(self.can.receive()).unwrap();

        let id = match frame_rx.id() {
            Id::Extended(id) => id.as_raw() as u8,
            Id::Standard(id) => id.as_raw() as u8,
        };

        let data: [u8; 7] = match (frame_rx.data().unwrap() as &[u8]).try_into() {
            Ok(data) => data,
            Err(_) => [0; 7],
        };

        async move {
            [
                id, data[0], data[1], data[2], data[3], data[4], data[5], data[6],
            ]
        }
    }
}
