#[doc = "Register `hcsparams` reader"]
pub struct R(crate::R<HCSPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCSPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCSPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCSPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `n_ports` reader - This field specifies the number of physical downstream ports implemented on this host controller. The value of this field determines how many port registers are addressable in the Operational Register Space. Valid values are in the range of 0x1 to 0x0f."]
pub type N_PORTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `port_routing_rules` reader - This field indicates the method used by this implementation for how all ports are mapped to companion controllers."]
pub type PORT_ROUTING_RULES_R = crate::BitReader<bool>;
#[doc = "Field `n_pcc` reader - Number of Port per Companion Controller (N_PCC)\n\nThis field indicates the number of ports supported per companion host controller host controller. It is used to indicate the port routing configuration to system software."]
pub type N_PCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `n_cc` reader - Number of Companion Controller (N_CC)\n\nThis field indicates the number of companion controllers associated with this USB2.0 host controller. A zero in this field indicates there are no companion host controllers. And a value larger than zero in this field indicates there are companion USB1.1 host controller(s)."]
pub type N_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `debug_port_number` reader - This register identifies which of the host controller ports is the debug port. The value is the port number (one based) of the debug port."]
pub type DEBUG_PORT_NUMBER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This field specifies the number of physical downstream ports implemented on this host controller. The value of this field determines how many port registers are addressable in the Operational Register Space. Valid values are in the range of 0x1 to 0x0f."]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - This field indicates the method used by this implementation for how all ports are mapped to companion controllers."]
    #[inline(always)]
    pub fn port_routing_rules(&self) -> PORT_ROUTING_RULES_R {
        PORT_ROUTING_RULES_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Port per Companion Controller (N_PCC)\n\nThis field indicates the number of ports supported per companion host controller host controller. It is used to indicate the port routing configuration to system software."]
    #[inline(always)]
    pub fn n_pcc(&self) -> N_PCC_R {
        N_PCC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Companion Controller (N_CC)\n\nThis field indicates the number of companion controllers associated with this USB2.0 host controller. A zero in this field indicates there are no companion host controllers. And a value larger than zero in this field indicates there are companion USB1.1 host controller(s)."]
    #[inline(always)]
    pub fn n_cc(&self) -> N_CC_R {
        N_CC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - This register identifies which of the host controller ports is the debug port. The value is the port number (one based) of the debug port."]
    #[inline(always)]
    pub fn debug_port_number(&self) -> DEBUG_PORT_NUMBER_R {
        DEBUG_PORT_NUMBER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "EHCI Host Control Structural Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](index.html) module"]
pub struct HCSPARAMS_SPEC;
impl crate::RegisterSpec for HCSPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcsparams::R](R) reader structure"]
impl crate::Readable for HCSPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets hcsparams to value 0"]
impl crate::Resettable for HCSPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
