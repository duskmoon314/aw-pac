#[doc = "Register `hcsp_portroute` reader"]
pub type R = crate::R<HCSP_PORTROUTE_SPEC>;
#[doc = "Field `hcsp_portroute` reader - This optional field is valid only if Port Routing Rules field in HCSPARAMS register is set to a one.\n\nThis field is used to allow a host controller implementation to explicitly describe to which companion host controller each implemented port is mapped. This field is a 15-element nibble array (each 4 bit is one array element). Each array location corresponds one-to-one with a physical port provided by the host controller (e.g. PORTROUTE \\[0\\] corresponds to the first PORTSC port, PORTROUTE \\[1\\] to the second PORTSC port, etc.). The value of each element indicates to which the companion host controllers this port is routed. Only the first N_PORTS elements have valid information. A value of zero indicates that the port is routed to the lowest numbered function companion host controller. A value of one indicates that the port is routed to the next lowest numbered function companion host controller, and so on."]
pub type HCSP_PORTROUTE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This optional field is valid only if Port Routing Rules field in HCSPARAMS register is set to a one.\n\nThis field is used to allow a host controller implementation to explicitly describe to which companion host controller each implemented port is mapped. This field is a 15-element nibble array (each 4 bit is one array element). Each array location corresponds one-to-one with a physical port provided by the host controller (e.g. PORTROUTE \\[0\\] corresponds to the first PORTSC port, PORTROUTE \\[1\\] to the second PORTSC port, etc.). The value of each element indicates to which the companion host controllers this port is routed. Only the first N_PORTS elements have valid information. A value of zero indicates that the port is routed to the lowest numbered function companion host controller. A value of one indicates that the port is routed to the next lowest numbered function companion host controller, and so on."]
    #[inline(always)]
    pub fn hcsp_portroute(&self) -> HCSP_PORTROUTE_R {
        HCSP_PORTROUTE_R::new(self.bits)
    }
}
#[doc = "EHCI Companion Port Route Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsp_portroute::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCSP_PORTROUTE_SPEC;
impl crate::RegisterSpec for HCSP_PORTROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsp_portroute::R`](R) reader structure"]
impl crate::Readable for HCSP_PORTROUTE_SPEC {}
#[doc = "`reset()` method sets hcsp_portroute to value 0"]
impl crate::Resettable for HCSP_PORTROUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
