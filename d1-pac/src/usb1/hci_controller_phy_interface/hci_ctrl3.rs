#[doc = "Register `hci_ctrl3` reader"]
pub struct R(crate::R<HCI_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCI_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCI_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCI_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hci_ctrl3` writer"]
pub struct W(crate::W<HCI_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCI_CTRL3_SPEC>;
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
impl From<crate::W<HCI_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCI_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `linestate_change_detect_enable` reader - Linestate Change Detect Enable"]
pub type LINESTATE_CHANGE_DETECT_ENABLE_R = crate::BitReader<LINESTATE_CHANGE_DETECT_ENABLE_A>;
#[doc = "Linestate Change Detect Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINESTATE_CHANGE_DETECT_ENABLE_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<LINESTATE_CHANGE_DETECT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATE_CHANGE_DETECT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINESTATE_CHANGE_DETECT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATE_CHANGE_DETECT_ENABLE_A {
        match self.bits {
            true => LINESTATE_CHANGE_DETECT_ENABLE_A::ENABLE,
            false => LINESTATE_CHANGE_DETECT_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LINESTATE_CHANGE_DETECT_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LINESTATE_CHANGE_DETECT_ENABLE_A::DISABLE
    }
}
#[doc = "Field `linestate_change_detect_enable` writer - Linestate Change Detect Enable"]
pub type LINESTATE_CHANGE_DETECT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_CTRL3_SPEC, LINESTATE_CHANGE_DETECT_ENABLE_A, O>;
impl<'a, const O: u8> LINESTATE_CHANGE_DETECT_ENABLE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_DETECT_ENABLE_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_DETECT_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `linestate_change_interrupt_enable` reader - Linestate Change Interrupt Enable"]
pub type LINESTATE_CHANGE_INTERRUPT_ENABLE_R =
    crate::BitReader<LINESTATE_CHANGE_INTERRUPT_ENABLE_A>;
#[doc = "Linestate Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINESTATE_CHANGE_INTERRUPT_ENABLE_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<LINESTATE_CHANGE_INTERRUPT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATE_CHANGE_INTERRUPT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINESTATE_CHANGE_INTERRUPT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATE_CHANGE_INTERRUPT_ENABLE_A {
        match self.bits {
            true => LINESTATE_CHANGE_INTERRUPT_ENABLE_A::ENABLE,
            false => LINESTATE_CHANGE_INTERRUPT_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LINESTATE_CHANGE_INTERRUPT_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LINESTATE_CHANGE_INTERRUPT_ENABLE_A::DISABLE
    }
}
#[doc = "Field `linestate_change_interrupt_enable` writer - Linestate Change Interrupt Enable"]
pub type LINESTATE_CHANGE_INTERRUPT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_CTRL3_SPEC, LINESTATE_CHANGE_INTERRUPT_ENABLE_A, O>;
impl<'a, const O: u8> LINESTATE_CHANGE_INTERRUPT_ENABLE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_INTERRUPT_ENABLE_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_INTERRUPT_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `remote_wakeup_enable` reader - Remote Wakeup Enable"]
pub type REMOTE_WAKEUP_ENABLE_R = crate::BitReader<REMOTE_WAKEUP_ENABLE_A>;
#[doc = "Remote Wakeup Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REMOTE_WAKEUP_ENABLE_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<REMOTE_WAKEUP_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: REMOTE_WAKEUP_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl REMOTE_WAKEUP_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REMOTE_WAKEUP_ENABLE_A {
        match self.bits {
            true => REMOTE_WAKEUP_ENABLE_A::ENABLE,
            false => REMOTE_WAKEUP_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REMOTE_WAKEUP_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REMOTE_WAKEUP_ENABLE_A::DISABLE
    }
}
#[doc = "Field `remote_wakeup_enable` writer - Remote Wakeup Enable"]
pub type REMOTE_WAKEUP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_CTRL3_SPEC, REMOTE_WAKEUP_ENABLE_A, O>;
impl<'a, const O: u8> REMOTE_WAKEUP_ENABLE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REMOTE_WAKEUP_ENABLE_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REMOTE_WAKEUP_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `linestate_change_detect` reader - Linestate Change Detect"]
pub type LINESTATE_CHANGE_DETECT_R = crate::BitReader<LINESTATE_CHANGE_DETECT_A>;
#[doc = "Linestate Change Detect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINESTATE_CHANGE_DETECT_A {
    #[doc = "0: Linestate change not dected"]
    NOT_DECTED = 0,
    #[doc = "1: Linestate change dected Write '1' to clear."]
    DECTED = 1,
}
impl From<LINESTATE_CHANGE_DETECT_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATE_CHANGE_DETECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LINESTATE_CHANGE_DETECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATE_CHANGE_DETECT_A {
        match self.bits {
            false => LINESTATE_CHANGE_DETECT_A::NOT_DECTED,
            true => LINESTATE_CHANGE_DETECT_A::DECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DECTED`"]
    #[inline(always)]
    pub fn is_not_dected(&self) -> bool {
        *self == LINESTATE_CHANGE_DETECT_A::NOT_DECTED
    }
    #[doc = "Checks if the value of the field is `DECTED`"]
    #[inline(always)]
    pub fn is_dected(&self) -> bool {
        *self == LINESTATE_CHANGE_DETECT_A::DECTED
    }
}
#[doc = "Field `linestate_change_detect` writer - Linestate Change Detect"]
pub type LINESTATE_CHANGE_DETECT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HCI_CTRL3_SPEC, LINESTATE_CHANGE_DETECT_A, O>;
impl<'a, const O: u8> LINESTATE_CHANGE_DETECT_W<'a, O> {
    #[doc = "Linestate change not dected"]
    #[inline(always)]
    pub fn not_dected(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_DETECT_A::NOT_DECTED)
    }
    #[doc = "Linestate change dected Write '1' to clear."]
    #[inline(always)]
    pub fn dected(self) -> &'a mut W {
        self.variant(LINESTATE_CHANGE_DETECT_A::DECTED)
    }
}
impl R {
    #[doc = "Bit 0 - Linestate Change Detect Enable"]
    #[inline(always)]
    pub fn linestate_change_detect_enable(&self) -> LINESTATE_CHANGE_DETECT_ENABLE_R {
        LINESTATE_CHANGE_DETECT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Linestate Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestate_change_interrupt_enable(&self) -> LINESTATE_CHANGE_INTERRUPT_ENABLE_R {
        LINESTATE_CHANGE_INTERRUPT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Remote Wakeup Enable"]
    #[inline(always)]
    pub fn remote_wakeup_enable(&self) -> REMOTE_WAKEUP_ENABLE_R {
        REMOTE_WAKEUP_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Linestate Change Detect"]
    #[inline(always)]
    pub fn linestate_change_detect(&self) -> LINESTATE_CHANGE_DETECT_R {
        LINESTATE_CHANGE_DETECT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Linestate Change Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linestate_change_detect_enable(&mut self) -> LINESTATE_CHANGE_DETECT_ENABLE_W<0> {
        LINESTATE_CHANGE_DETECT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Linestate Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linestate_change_interrupt_enable(&mut self) -> LINESTATE_CHANGE_INTERRUPT_ENABLE_W<1> {
        LINESTATE_CHANGE_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Remote Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn remote_wakeup_enable(&mut self) -> REMOTE_WAKEUP_ENABLE_W<3> {
        REMOTE_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Linestate Change Detect"]
    #[inline(always)]
    #[must_use]
    pub fn linestate_change_detect(&mut self) -> LINESTATE_CHANGE_DETECT_W<16> {
        LINESTATE_CHANGE_DETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HCI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hci_ctrl3](index.html) module"]
pub struct HCI_CTRL3_SPEC;
impl crate::RegisterSpec for HCI_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hci_ctrl3::R](R) reader structure"]
impl crate::Readable for HCI_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hci_ctrl3::W](W) writer structure"]
impl crate::Writable for HCI_CTRL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_0000;
}
#[doc = "`reset()` method sets hci_ctrl3 to value 0x0001_0000"]
impl crate::Resettable for HCI_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
