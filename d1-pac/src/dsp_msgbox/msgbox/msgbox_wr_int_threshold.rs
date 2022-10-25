#[doc = "Register `msgbox_wr_int_threshold%s` reader"]
pub struct R(crate::R<MSGBOX_WR_INT_THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_WR_INT_THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_WR_INT_THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_WR_INT_THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msgbox_wr_int_threshold%s` writer"]
pub struct W(crate::W<MSGBOX_WR_INT_THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_WR_INT_THRESHOLD_SPEC>;
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
impl From<crate::W<MSGBOX_WR_INT_THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_WR_INT_THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `msg_wr_int_threshold_cfg` reader - Configure the FIFO empty level to trigger the write interrupt for user1"]
pub type MSG_WR_INT_THRESHOLD_CFG_R = crate::FieldReader<u8, MSG_WR_INT_THRESHOLD_CFG_A>;
#[doc = "Configure the FIFO empty level to trigger the write interrupt for user1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSG_WR_INT_THRESHOLD_CFG_A {
    #[doc = "0: `0`"]
    T1 = 0,
    #[doc = "1: `1`"]
    T2 = 1,
    #[doc = "2: `10`"]
    T4 = 2,
    #[doc = "3: `11`"]
    T8 = 3,
}
impl From<MSG_WR_INT_THRESHOLD_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MSG_WR_INT_THRESHOLD_CFG_A) -> Self {
        variant as _
    }
}
impl MSG_WR_INT_THRESHOLD_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSG_WR_INT_THRESHOLD_CFG_A {
        match self.bits {
            0 => MSG_WR_INT_THRESHOLD_CFG_A::T1,
            1 => MSG_WR_INT_THRESHOLD_CFG_A::T2,
            2 => MSG_WR_INT_THRESHOLD_CFG_A::T4,
            3 => MSG_WR_INT_THRESHOLD_CFG_A::T8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T1`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T1
    }
    #[doc = "Checks if the value of the field is `T2`"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T2
    }
    #[doc = "Checks if the value of the field is `T4`"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T4
    }
    #[doc = "Checks if the value of the field is `T8`"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T8
    }
}
#[doc = "Field `msg_wr_int_threshold_cfg` writer - Configure the FIFO empty level to trigger the write interrupt for user1"]
pub type MSG_WR_INT_THRESHOLD_CFG_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    MSGBOX_WR_INT_THRESHOLD_SPEC,
    u8,
    MSG_WR_INT_THRESHOLD_CFG_A,
    2,
    O,
>;
impl<'a, const O: u8> MSG_WR_INT_THRESHOLD_CFG_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t1(self) -> &'a mut W {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t2(self) -> &'a mut W {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut W {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut W {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure the FIFO empty level to trigger the write interrupt for user1"]
    #[inline(always)]
    pub fn msg_wr_int_threshold_cfg(&self) -> MSG_WR_INT_THRESHOLD_CFG_R {
        MSG_WR_INT_THRESHOLD_CFG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the FIFO empty level to trigger the write interrupt for user1"]
    #[inline(always)]
    #[must_use]
    pub fn msg_wr_int_threshold_cfg(&mut self) -> MSG_WR_INT_THRESHOLD_CFG_W<0> {
        MSG_WR_INT_THRESHOLD_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Box Write Interrupt Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_wr_int_threshold](index.html) module"]
pub struct MSGBOX_WR_INT_THRESHOLD_SPEC;
impl crate::RegisterSpec for MSGBOX_WR_INT_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_wr_int_threshold::R](R) reader structure"]
impl crate::Readable for MSGBOX_WR_INT_THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_wr_int_threshold::W](W) writer structure"]
impl crate::Writable for MSGBOX_WR_INT_THRESHOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_wr_int_threshold%s to value 0"]
impl crate::Resettable for MSGBOX_WR_INT_THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
