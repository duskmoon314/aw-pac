#[doc = "Register `hc_rh_descriptor_a` reader"]
pub struct R(crate::R<HC_RH_DESCRIPTOR_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_RH_DESCRIPTOR_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_RH_DESCRIPTOR_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_RH_DESCRIPTOR_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_rh_descriptor_a` writer"]
pub struct W(crate::W<HC_RH_DESCRIPTOR_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_RH_DESCRIPTOR_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HC_RH_DESCRIPTOR_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_RH_DESCRIPTOR_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `number_downstream_ports` reader - NumberDownstreamPorts\n\nThese bits specify the number of downstream ports supported by the Root Hub. It is implementation-specific. The minimum number of ports is 1. The maximum number of ports supported."]
pub type NUMBER_DOWNSTREAM_PORTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `no_power_swithcing` reader - NoPowerSwithcing\n\nThese bits are used to specify whether power switching is supported or ports are always powered. It is implementation- specific. When this bit is cleared, the PowerSwitchingMode specifies global or per-port switching."]
pub type NO_POWER_SWITHCING_R = crate::BitReader<NO_POWER_SWITHCING_A>;
#[doc = "NoPowerSwithcing\n\nThese bits are used to specify whether power switching is supported or ports are always powered. It is implementation- specific. When this bit is cleared, the PowerSwitchingMode specifies global or per-port switching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NO_POWER_SWITHCING_A {
    #[doc = "0: Ports are power switched"]
    SWITCHED = 0,
    #[doc = "1: Ports are always powered on when the HC is powered on"]
    ALWAYS = 1,
}
impl From<NO_POWER_SWITHCING_A> for bool {
    #[inline(always)]
    fn from(variant: NO_POWER_SWITHCING_A) -> Self {
        variant as u8 != 0
    }
}
impl NO_POWER_SWITHCING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_POWER_SWITHCING_A {
        match self.bits {
            false => NO_POWER_SWITHCING_A::SWITCHED,
            true => NO_POWER_SWITHCING_A::ALWAYS,
        }
    }
    #[doc = "Checks if the value of the field is `SWITCHED`"]
    #[inline(always)]
    pub fn is_switched(&self) -> bool {
        *self == NO_POWER_SWITHCING_A::SWITCHED
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == NO_POWER_SWITHCING_A::ALWAYS
    }
}
#[doc = "Field `no_power_swithcing` writer - NoPowerSwithcing\n\nThese bits are used to specify whether power switching is supported or ports are always powered. It is implementation- specific. When this bit is cleared, the PowerSwitchingMode specifies global or per-port switching."]
pub type NO_POWER_SWITHCING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_DESCRIPTOR_A_SPEC, NO_POWER_SWITHCING_A, O>;
impl<'a, const O: u8> NO_POWER_SWITHCING_W<'a, O> {
    #[doc = "Ports are power switched"]
    #[inline(always)]
    pub fn switched(self) -> &'a mut W {
        self.variant(NO_POWER_SWITHCING_A::SWITCHED)
    }
    #[doc = "Ports are always powered on when the HC is powered on"]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(NO_POWER_SWITHCING_A::ALWAYS)
    }
}
#[doc = "Field `power_switching_mode` reader - PowerSwitchingMode\n\nThis bit is used to specify how the power switching of the Root Hub ports is controlled. It is implementation-specific. This field is only valid if the NoPowerSwitching field is cleared."]
pub type POWER_SWITCHING_MODE_R = crate::BitReader<POWER_SWITCHING_MODE_A>;
#[doc = "PowerSwitchingMode\n\nThis bit is used to specify how the power switching of the Root Hub ports is controlled. It is implementation-specific. This field is only valid if the NoPowerSwitching field is cleared.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWER_SWITCHING_MODE_A {
    #[doc = "0: All ports are powered at the same time"]
    SAME_TIME = 0,
    #[doc = "1: Each port is powered individually. This mode allows port power to be controlled by either the global switch or per- port switching. If the PortPowerControlMask bit is set, the port responds only to port power commands ( Set/ClearPortPower ). If the port mask is cleared, then the port is controlled only by the global power switch ( Set/ClearGlobalPower )."]
    INDIVIDUAL = 1,
}
impl From<POWER_SWITCHING_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: POWER_SWITCHING_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl POWER_SWITCHING_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_SWITCHING_MODE_A {
        match self.bits {
            false => POWER_SWITCHING_MODE_A::SAME_TIME,
            true => POWER_SWITCHING_MODE_A::INDIVIDUAL,
        }
    }
    #[doc = "Checks if the value of the field is `SAME_TIME`"]
    #[inline(always)]
    pub fn is_same_time(&self) -> bool {
        *self == POWER_SWITCHING_MODE_A::SAME_TIME
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == POWER_SWITCHING_MODE_A::INDIVIDUAL
    }
}
#[doc = "Field `power_switching_mode` writer - PowerSwitchingMode\n\nThis bit is used to specify how the power switching of the Root Hub ports is controlled. It is implementation-specific. This field is only valid if the NoPowerSwitching field is cleared."]
pub type POWER_SWITCHING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_DESCRIPTOR_A_SPEC, POWER_SWITCHING_MODE_A, O>;
impl<'a, const O: u8> POWER_SWITCHING_MODE_W<'a, O> {
    #[doc = "All ports are powered at the same time"]
    #[inline(always)]
    pub fn same_time(self) -> &'a mut W {
        self.variant(POWER_SWITCHING_MODE_A::SAME_TIME)
    }
    #[doc = "Each port is powered individually. This mode allows port power to be controlled by either the global switch or per- port switching. If the PortPowerControlMask bit is set, the port responds only to port power commands ( Set/ClearPortPower ). If the port mask is cleared, then the port is controlled only by the global power switch ( Set/ClearGlobalPower )."]
    #[inline(always)]
    pub fn individual(self) -> &'a mut W {
        self.variant(POWER_SWITCHING_MODE_A::INDIVIDUAL)
    }
}
#[doc = "Field `device` reader - Device Type This bit specifies that the Root Hub is not a compound device. The Root Hub is not permitted to be a compound device. This field should always read/write 0."]
pub type DEVICE_R = crate::BitReader<bool>;
#[doc = "Field `over_current_protection_mode` reader - OverCurrentProtectionMode\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. At reset, these fields should reflect the same mode as PowerSwitchingMode. This field is valid only if the NoOverCurrentProtection field is cleared. 0 Over-current status is reported collectively for all downstream ports. 1 Over-current status is reported on per-port basis."]
pub type OVER_CURRENT_PROTECTION_MODE_R = crate::BitReader<OVER_CURRENT_PROTECTION_MODE_A>;
#[doc = "OverCurrentProtectionMode\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. At reset, these fields should reflect the same mode as PowerSwitchingMode. This field is valid only if the NoOverCurrentProtection field is cleared. 0 Over-current status is reported collectively for all downstream ports. 1 Over-current status is reported on per-port basis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER_CURRENT_PROTECTION_MODE_A {
    #[doc = "0: Over-current status is reported collectively for all downstream ports"]
    REPORT = 0,
    #[doc = "1: Over-current status is reported on per-port basis"]
    REPORT_PERPORT = 1,
}
impl From<OVER_CURRENT_PROTECTION_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_CURRENT_PROTECTION_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER_CURRENT_PROTECTION_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_CURRENT_PROTECTION_MODE_A {
        match self.bits {
            false => OVER_CURRENT_PROTECTION_MODE_A::REPORT,
            true => OVER_CURRENT_PROTECTION_MODE_A::REPORT_PERPORT,
        }
    }
    #[doc = "Checks if the value of the field is `REPORT`"]
    #[inline(always)]
    pub fn is_report(&self) -> bool {
        *self == OVER_CURRENT_PROTECTION_MODE_A::REPORT
    }
    #[doc = "Checks if the value of the field is `REPORT_PERPORT`"]
    #[inline(always)]
    pub fn is_report_perport(&self) -> bool {
        *self == OVER_CURRENT_PROTECTION_MODE_A::REPORT_PERPORT
    }
}
#[doc = "Field `over_current_protection_mode` writer - OverCurrentProtectionMode\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. At reset, these fields should reflect the same mode as PowerSwitchingMode. This field is valid only if the NoOverCurrentProtection field is cleared. 0 Over-current status is reported collectively for all downstream ports. 1 Over-current status is reported on per-port basis."]
pub type OVER_CURRENT_PROTECTION_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_DESCRIPTOR_A_SPEC, OVER_CURRENT_PROTECTION_MODE_A, O>;
impl<'a, const O: u8> OVER_CURRENT_PROTECTION_MODE_W<'a, O> {
    #[doc = "Over-current status is reported collectively for all downstream ports"]
    #[inline(always)]
    pub fn report(self) -> &'a mut W {
        self.variant(OVER_CURRENT_PROTECTION_MODE_A::REPORT)
    }
    #[doc = "Over-current status is reported on per-port basis"]
    #[inline(always)]
    pub fn report_perport(self) -> &'a mut W {
        self.variant(OVER_CURRENT_PROTECTION_MODE_A::REPORT_PERPORT)
    }
}
#[doc = "Field `no_over_current_protection` reader - NoOverCurrentProtection\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. When this bit is cleared, the OverCurrentProtectionMode field specifies global or per-port reporting."]
pub type NO_OVER_CURRENT_PROTECTION_R = crate::BitReader<NO_OVER_CURRENT_PROTECTION_A>;
#[doc = "NoOverCurrentProtection\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. When this bit is cleared, the OverCurrentProtectionMode field specifies global or per-port reporting.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NO_OVER_CURRENT_PROTECTION_A {
    #[doc = "0: Over-current status is reported collectively for all downstream ports"]
    REPORT = 0,
    #[doc = "1: No over-current protection supported"]
    NO_SUPPORT = 1,
}
impl From<NO_OVER_CURRENT_PROTECTION_A> for bool {
    #[inline(always)]
    fn from(variant: NO_OVER_CURRENT_PROTECTION_A) -> Self {
        variant as u8 != 0
    }
}
impl NO_OVER_CURRENT_PROTECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_OVER_CURRENT_PROTECTION_A {
        match self.bits {
            false => NO_OVER_CURRENT_PROTECTION_A::REPORT,
            true => NO_OVER_CURRENT_PROTECTION_A::NO_SUPPORT,
        }
    }
    #[doc = "Checks if the value of the field is `REPORT`"]
    #[inline(always)]
    pub fn is_report(&self) -> bool {
        *self == NO_OVER_CURRENT_PROTECTION_A::REPORT
    }
    #[doc = "Checks if the value of the field is `NO_SUPPORT`"]
    #[inline(always)]
    pub fn is_no_support(&self) -> bool {
        *self == NO_OVER_CURRENT_PROTECTION_A::NO_SUPPORT
    }
}
#[doc = "Field `no_over_current_protection` writer - NoOverCurrentProtection\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. When this bit is cleared, the OverCurrentProtectionMode field specifies global or per-port reporting."]
pub type NO_OVER_CURRENT_PROTECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HC_RH_DESCRIPTOR_A_SPEC, NO_OVER_CURRENT_PROTECTION_A, O>;
impl<'a, const O: u8> NO_OVER_CURRENT_PROTECTION_W<'a, O> {
    #[doc = "Over-current status is reported collectively for all downstream ports"]
    #[inline(always)]
    pub fn report(self) -> &'a mut W {
        self.variant(NO_OVER_CURRENT_PROTECTION_A::REPORT)
    }
    #[doc = "No over-current protection supported"]
    #[inline(always)]
    pub fn no_support(self) -> &'a mut W {
        self.variant(NO_OVER_CURRENT_PROTECTION_A::NO_SUPPORT)
    }
}
#[doc = "Field `power_on_to_power_good_time` reader - PowerOnToPowerGoodTime\\[POTPGT\\]\n\nThis byte specifies the duration HCD has to wait before accessing a powered-on port of the Root Hub. It is implementation-specific. The unit of time is 2 ms. The duration is calculated as POTPGT * 2ms."]
pub type POWER_ON_TO_POWER_GOOD_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `power_on_to_power_good_time` writer - PowerOnToPowerGoodTime\\[POTPGT\\]\n\nThis byte specifies the duration HCD has to wait before accessing a powered-on port of the Root Hub. It is implementation-specific. The unit of time is 2 ms. The duration is calculated as POTPGT * 2ms."]
pub type POWER_ON_TO_POWER_GOOD_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_RH_DESCRIPTOR_A_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NumberDownstreamPorts\n\nThese bits specify the number of downstream ports supported by the Root Hub. It is implementation-specific. The minimum number of ports is 1. The maximum number of ports supported."]
    #[inline(always)]
    pub fn number_downstream_ports(&self) -> NUMBER_DOWNSTREAM_PORTS_R {
        NUMBER_DOWNSTREAM_PORTS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - NoPowerSwithcing\n\nThese bits are used to specify whether power switching is supported or ports are always powered. It is implementation- specific. When this bit is cleared, the PowerSwitchingMode specifies global or per-port switching."]
    #[inline(always)]
    pub fn no_power_swithcing(&self) -> NO_POWER_SWITHCING_R {
        NO_POWER_SWITHCING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PowerSwitchingMode\n\nThis bit is used to specify how the power switching of the Root Hub ports is controlled. It is implementation-specific. This field is only valid if the NoPowerSwitching field is cleared."]
    #[inline(always)]
    pub fn power_switching_mode(&self) -> POWER_SWITCHING_MODE_R {
        POWER_SWITCHING_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Device Type This bit specifies that the Root Hub is not a compound device. The Root Hub is not permitted to be a compound device. This field should always read/write 0."]
    #[inline(always)]
    pub fn device(&self) -> DEVICE_R {
        DEVICE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. At reset, these fields should reflect the same mode as PowerSwitchingMode. This field is valid only if the NoOverCurrentProtection field is cleared. 0 Over-current status is reported collectively for all downstream ports. 1 Over-current status is reported on per-port basis."]
    #[inline(always)]
    pub fn over_current_protection_mode(&self) -> OVER_CURRENT_PROTECTION_MODE_R {
        OVER_CURRENT_PROTECTION_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. When this bit is cleared, the OverCurrentProtectionMode field specifies global or per-port reporting."]
    #[inline(always)]
    pub fn no_over_current_protection(&self) -> NO_OVER_CURRENT_PROTECTION_R {
        NO_OVER_CURRENT_PROTECTION_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime\\[POTPGT\\]\n\nThis byte specifies the duration HCD has to wait before accessing a powered-on port of the Root Hub. It is implementation-specific. The unit of time is 2 ms. The duration is calculated as POTPGT * 2ms."]
    #[inline(always)]
    pub fn power_on_to_power_good_time(&self) -> POWER_ON_TO_POWER_GOOD_TIME_R {
        POWER_ON_TO_POWER_GOOD_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - NoPowerSwithcing\n\nThese bits are used to specify whether power switching is supported or ports are always powered. It is implementation- specific. When this bit is cleared, the PowerSwitchingMode specifies global or per-port switching."]
    #[inline(always)]
    #[must_use]
    pub fn no_power_swithcing(&mut self) -> NO_POWER_SWITHCING_W<8> {
        NO_POWER_SWITHCING_W::new(self)
    }
    #[doc = "Bit 9 - PowerSwitchingMode\n\nThis bit is used to specify how the power switching of the Root Hub ports is controlled. It is implementation-specific. This field is only valid if the NoPowerSwitching field is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn power_switching_mode(&mut self) -> POWER_SWITCHING_MODE_W<9> {
        POWER_SWITCHING_MODE_W::new(self)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. At reset, these fields should reflect the same mode as PowerSwitchingMode. This field is valid only if the NoOverCurrentProtection field is cleared. 0 Over-current status is reported collectively for all downstream ports. 1 Over-current status is reported on per-port basis."]
    #[inline(always)]
    #[must_use]
    pub fn over_current_protection_mode(&mut self) -> OVER_CURRENT_PROTECTION_MODE_W<11> {
        OVER_CURRENT_PROTECTION_MODE_W::new(self)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection\n\nThis bit describes how the overcurrent status for the Root Hub ports are reported. When this bit is cleared, the OverCurrentProtectionMode field specifies global or per-port reporting."]
    #[inline(always)]
    #[must_use]
    pub fn no_over_current_protection(&mut self) -> NO_OVER_CURRENT_PROTECTION_W<12> {
        NO_OVER_CURRENT_PROTECTION_W::new(self)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime\\[POTPGT\\]\n\nThis byte specifies the duration HCD has to wait before accessing a powered-on port of the Root Hub. It is implementation-specific. The unit of time is 2 ms. The duration is calculated as POTPGT * 2ms."]
    #[inline(always)]
    #[must_use]
    pub fn power_on_to_power_good_time(&mut self) -> POWER_ON_TO_POWER_GOOD_TIME_W<24> {
        POWER_ON_TO_POWER_GOOD_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Root Hub Descriptor Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_rh_descriptor_a](index.html) module"]
pub struct HC_RH_DESCRIPTOR_A_SPEC;
impl crate::RegisterSpec for HC_RH_DESCRIPTOR_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_rh_descriptor_a::R](R) reader structure"]
impl crate::Readable for HC_RH_DESCRIPTOR_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_rh_descriptor_a::W](W) writer structure"]
impl crate::Writable for HC_RH_DESCRIPTOR_A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_rh_descriptor_a to value 0x0200_1201"]
impl crate::Resettable for HC_RH_DESCRIPTOR_A_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_1201;
}
