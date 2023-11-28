#[doc = "Register `hci_sie_port_disable_control` reader"]
pub type R = crate::R<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
#[doc = "Register `hci_sie_port_disable_control` writer"]
pub type W = crate::W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
#[doc = "Field `port_disable_control` reader - Port Disable Control"]
pub type PORT_DISABLE_CONTROL_R = crate::FieldReader<PORT_DISABLE_CONTROL_A>;
#[doc = "Port Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PORT_DISABLE_CONTROL_A {
    #[doc = "0: Port Disable when no-se0 detect before SOF"]
    DISABLE = 0,
    #[doc = "2: No Port Disable when no-se0 detect before SOF"]
    NO_DISABLE = 2,
    #[doc = "3: Port Disable when no-se0 3 time detect before SOF during 8 frames"]
    DISABLE_3_TIME_DETECT = 3,
}
impl From<PORT_DISABLE_CONTROL_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_DISABLE_CONTROL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PORT_DISABLE_CONTROL_A {
    type Ux = u8;
}
impl PORT_DISABLE_CONTROL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PORT_DISABLE_CONTROL_A> {
        match self.bits {
            0 => Some(PORT_DISABLE_CONTROL_A::DISABLE),
            2 => Some(PORT_DISABLE_CONTROL_A::NO_DISABLE),
            3 => Some(PORT_DISABLE_CONTROL_A::DISABLE_3_TIME_DETECT),
            _ => None,
        }
    }
    #[doc = "Port Disable when no-se0 detect before SOF"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PORT_DISABLE_CONTROL_A::DISABLE
    }
    #[doc = "No Port Disable when no-se0 detect before SOF"]
    #[inline(always)]
    pub fn is_no_disable(&self) -> bool {
        *self == PORT_DISABLE_CONTROL_A::NO_DISABLE
    }
    #[doc = "Port Disable when no-se0 3 time detect before SOF during 8 frames"]
    #[inline(always)]
    pub fn is_disable_3_time_detect(&self) -> bool {
        *self == PORT_DISABLE_CONTROL_A::DISABLE_3_TIME_DETECT
    }
}
#[doc = "Field `port_disable_control` writer - Port Disable Control"]
pub type PORT_DISABLE_CONTROL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PORT_DISABLE_CONTROL_A>;
impl<'a, REG> PORT_DISABLE_CONTROL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port Disable when no-se0 detect before SOF"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PORT_DISABLE_CONTROL_A::DISABLE)
    }
    #[doc = "No Port Disable when no-se0 detect before SOF"]
    #[inline(always)]
    pub fn no_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PORT_DISABLE_CONTROL_A::NO_DISABLE)
    }
    #[doc = "Port Disable when no-se0 3 time detect before SOF during 8 frames"]
    #[inline(always)]
    pub fn disable_3_time_detect(self) -> &'a mut crate::W<REG> {
        self.variant(PORT_DISABLE_CONTROL_A::DISABLE_3_TIME_DETECT)
    }
}
#[doc = "Field `resume_sel` reader - resume_sel\n\nWhen set k-se0 transition 2 us, setting this bit to 1, which is cooperated with ss_utmi_backward_enb_i."]
pub type RESUME_SEL_R = crate::BitReader;
#[doc = "Field `resume_sel` writer - resume_sel\n\nWhen set k-se0 transition 2 us, setting this bit to 1, which is cooperated with ss_utmi_backward_enb_i."]
pub type RESUME_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se0_status` reader - SE0 Status\n\nThis bit is set when no-se0 is detected before SOF when bit\\[1:0\\] is 10b or 11b"]
pub type SE0_STATUS_R = crate::BitReader;
#[doc = "Field `se0_status` writer - SE0 Status\n\nThis bit is set when no-se0 is detected before SOF when bit\\[1:0\\] is 10b or 11b"]
pub type SE0_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Port Disable Control"]
    #[inline(always)]
    pub fn port_disable_control(&self) -> PORT_DISABLE_CONTROL_R {
        PORT_DISABLE_CONTROL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - resume_sel\n\nWhen set k-se0 transition 2 us, setting this bit to 1, which is cooperated with ss_utmi_backward_enb_i."]
    #[inline(always)]
    pub fn resume_sel(&self) -> RESUME_SEL_R {
        RESUME_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - SE0 Status\n\nThis bit is set when no-se0 is detected before SOF when bit\\[1:0\\] is 10b or 11b"]
    #[inline(always)]
    pub fn se0_status(&self) -> SE0_STATUS_R {
        SE0_STATUS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port Disable Control"]
    #[inline(always)]
    #[must_use]
    pub fn port_disable_control(
        &mut self,
    ) -> PORT_DISABLE_CONTROL_W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC> {
        PORT_DISABLE_CONTROL_W::new(self, 0)
    }
    #[doc = "Bit 4 - resume_sel\n\nWhen set k-se0 transition 2 us, setting this bit to 1, which is cooperated with ss_utmi_backward_enb_i."]
    #[inline(always)]
    #[must_use]
    pub fn resume_sel(&mut self) -> RESUME_SEL_W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC> {
        RESUME_SEL_W::new(self, 4)
    }
    #[doc = "Bit 16 - SE0 Status\n\nThis bit is set when no-se0 is detected before SOF when bit\\[1:0\\] is 10b or 11b"]
    #[inline(always)]
    #[must_use]
    pub fn se0_status(&mut self) -> SE0_STATUS_W<HCI_SIE_PORT_DISABLE_CONTROL_SPEC> {
        SE0_STATUS_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HCI SIE Port Disable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hci_sie_port_disable_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hci_sie_port_disable_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCI_SIE_PORT_DISABLE_CONTROL_SPEC;
impl crate::RegisterSpec for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hci_sie_port_disable_control::R`](R) reader structure"]
impl crate::Readable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hci_sie_port_disable_control::W`](W) writer structure"]
impl crate::Writable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hci_sie_port_disable_control to value 0"]
impl crate::Resettable for HCI_SIE_PORT_DISABLE_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
