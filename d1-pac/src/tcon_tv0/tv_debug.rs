#[doc = "Register `tv_debug` reader"]
pub type R = crate::R<TV_DEBUG_SPEC>;
#[doc = "Register `tv_debug` writer"]
pub type W = crate::W<TV_DEBUG_SPEC>;
#[doc = "Field `tv_current_line` reader - TV Current Line"]
pub type TV_CURRENT_LINE_R = crate::FieldReader<u16>;
#[doc = "Field `line_buf_bypass` reader - Line Buf fer Bypass"]
pub type LINE_BUF_BYPASS_R = crate::BitReader<LINE_BUF_BYPASS_A>;
#[doc = "Line Buf fer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_BUF_BYPASS_A {
    #[doc = "0: Used"]
    U_SED = 0,
    #[doc = "1: Bypass"]
    BYPASS = 1,
}
impl From<LINE_BUF_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_BUF_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl LINE_BUF_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINE_BUF_BYPASS_A {
        match self.bits {
            false => LINE_BUF_BYPASS_A::U_SED,
            true => LINE_BUF_BYPASS_A::BYPASS,
        }
    }
    #[doc = "Used"]
    #[inline(always)]
    pub fn is_u_sed(&self) -> bool {
        *self == LINE_BUF_BYPASS_A::U_SED
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == LINE_BUF_BYPASS_A::BYPASS
    }
}
#[doc = "Field `line_buf_bypass` writer - Line Buf fer Bypass"]
pub type LINE_BUF_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, LINE_BUF_BYPASS_A>;
impl<'a, REG> LINE_BUF_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Used"]
    #[inline(always)]
    pub fn u_sed(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_BUF_BYPASS_A::U_SED)
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_BUF_BYPASS_A::BYPASS)
    }
}
#[doc = "Field `tv_field_pol` reader - TV Field Polarity"]
pub type TV_FIELD_POL_R = crate::BitReader<TV_FIELD_POL_A>;
#[doc = "TV Field Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_FIELD_POL_A {
    #[doc = "0: Second field"]
    SECOND = 0,
    #[doc = "1: First field"]
    FIRST = 1,
}
impl From<TV_FIELD_POL_A> for bool {
    #[inline(always)]
    fn from(variant: TV_FIELD_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_FIELD_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TV_FIELD_POL_A {
        match self.bits {
            false => TV_FIELD_POL_A::SECOND,
            true => TV_FIELD_POL_A::FIRST,
        }
    }
    #[doc = "Second field"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == TV_FIELD_POL_A::SECOND
    }
    #[doc = "First field"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == TV_FIELD_POL_A::FIRST
    }
}
#[doc = "Field `tv_fifo_u` reader - TV FIFO Underflow"]
pub type TV_FIFO_U_R = crate::BitReader;
#[doc = "Field `tv_fifo_u` writer - TV FIFO Underflow"]
pub type TV_FIFO_U_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - TV Current Line"]
    #[inline(always)]
    pub fn tv_current_line(&self) -> TV_CURRENT_LINE_R {
        TV_CURRENT_LINE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - Line Buf fer Bypass"]
    #[inline(always)]
    pub fn line_buf_bypass(&self) -> LINE_BUF_BYPASS_R {
        LINE_BUF_BYPASS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 28 - TV Field Polarity"]
    #[inline(always)]
    pub fn tv_field_pol(&self) -> TV_FIELD_POL_R {
        TV_FIELD_POL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - TV FIFO Underflow"]
    #[inline(always)]
    pub fn tv_fifo_u(&self) -> TV_FIFO_U_R {
        TV_FIFO_U_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Line Buf fer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn line_buf_bypass(&mut self) -> LINE_BUF_BYPASS_W<TV_DEBUG_SPEC> {
        LINE_BUF_BYPASS_W::new(self, 13)
    }
    #[doc = "Bit 30 - TV FIFO Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn tv_fifo_u(&mut self) -> TV_FIFO_U_W<TV_DEBUG_SPEC> {
        TV_FIFO_U_W::new(self, 30)
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
#[doc = "TV Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_DEBUG_SPEC;
impl crate::RegisterSpec for TV_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_debug::R`](R) reader structure"]
impl crate::Readable for TV_DEBUG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_debug::W`](W) writer structure"]
impl crate::Writable for TV_DEBUG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_debug to value 0"]
impl crate::Resettable for TV_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
