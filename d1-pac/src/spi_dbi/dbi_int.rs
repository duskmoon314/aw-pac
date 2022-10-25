#[doc = "Register `dbi_int` reader"]
pub struct R(crate::R<DBI_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dbi_int` writer"]
pub struct W(crate::W<DBI_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_INT_SPEC>;
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
impl From<crate::W<DBI_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `line_done_int_en` reader - "]
pub type LINE_DONE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `line_done_int_en` writer - "]
pub type LINE_DONE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `fram_done_int_en` reader - "]
pub type FRAM_DONE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `fram_done_int_en` writer - "]
pub type FRAM_DONE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `te_int_en` reader - "]
pub type TE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `te_int_en` writer - "]
pub type TE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `rd_done_int_en` reader - "]
pub type RD_DONE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rd_done_int_en` writer - "]
pub type RD_DONE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `timer_int_en` reader - "]
pub type TIMER_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `timer_int_en` writer - "]
pub type TIMER_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `dbi_fifo_full_int_en` reader - "]
pub type DBI_FIFO_FULL_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dbi_fifo_full_int_en` writer - "]
pub type DBI_FIFO_FULL_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `dbi_fifo_empty_int_en` reader - "]
pub type DBI_FIFO_EMPTY_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `dbi_fifo_empty_int_en` writer - "]
pub type DBI_FIFO_EMPTY_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `line_done_int` reader - "]
pub type LINE_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `line_done_int` writer - "]
pub type LINE_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `fram_done_int` reader - "]
pub type FRAM_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `fram_done_int` writer - "]
pub type FRAM_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `te_int` reader - "]
pub type TE_INT_R = crate::BitReader<bool>;
#[doc = "Field `te_int` writer - "]
pub type TE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `rd_done_int` reader - "]
pub type RD_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `rd_done_int` writer - "]
pub type RD_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `timer_int` reader - "]
pub type TIMER_INT_R = crate::BitReader<bool>;
#[doc = "Field `timer_int` writer - "]
pub type TIMER_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `dbi_fifo_full_int` reader - "]
pub type DBI_FIFO_FULL_INT_R = crate::BitReader<bool>;
#[doc = "Field `dbi_fifo_full_int` writer - "]
pub type DBI_FIFO_FULL_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
#[doc = "Field `dbi_fifo_empty_int` reader - "]
pub type DBI_FIFO_EMPTY_INT_R = crate::BitReader<bool>;
#[doc = "Field `dbi_fifo_empty_int` writer - "]
pub type DBI_FIFO_EMPTY_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_INT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_done_int_en(&self) -> LINE_DONE_INT_EN_R {
        LINE_DONE_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fram_done_int_en(&self) -> FRAM_DONE_INT_EN_R {
        FRAM_DONE_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn te_int_en(&self) -> TE_INT_EN_R {
        TE_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_done_int_en(&self) -> RD_DONE_INT_EN_R {
        RD_DONE_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer_int_en(&self) -> TIMER_INT_EN_R {
        TIMER_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dbi_fifo_full_int_en(&self) -> DBI_FIFO_FULL_INT_EN_R {
        DBI_FIFO_FULL_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int_en(&self) -> DBI_FIFO_EMPTY_INT_EN_R {
        DBI_FIFO_EMPTY_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn line_done_int(&self) -> LINE_DONE_INT_R {
        LINE_DONE_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fram_done_int(&self) -> FRAM_DONE_INT_R {
        FRAM_DONE_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn te_int(&self) -> TE_INT_R {
        TE_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rd_done_int(&self) -> RD_DONE_INT_R {
        RD_DONE_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timer_int(&self) -> TIMER_INT_R {
        TIMER_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dbi_fifo_full_int(&self) -> DBI_FIFO_FULL_INT_R {
        DBI_FIFO_FULL_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dbi_fifo_empty_int(&self) -> DBI_FIFO_EMPTY_INT_R {
        DBI_FIFO_EMPTY_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn line_done_int_en(&mut self) -> LINE_DONE_INT_EN_W<0> {
        LINE_DONE_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fram_done_int_en(&mut self) -> FRAM_DONE_INT_EN_W<1> {
        FRAM_DONE_INT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn te_int_en(&mut self) -> TE_INT_EN_W<2> {
        TE_INT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rd_done_int_en(&mut self) -> RD_DONE_INT_EN_W<3> {
        RD_DONE_INT_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn timer_int_en(&mut self) -> TIMER_INT_EN_W<4> {
        TIMER_INT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_fifo_full_int_en(&mut self) -> DBI_FIFO_FULL_INT_EN_W<5> {
        DBI_FIFO_FULL_INT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_fifo_empty_int_en(&mut self) -> DBI_FIFO_EMPTY_INT_EN_W<6> {
        DBI_FIFO_EMPTY_INT_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn line_done_int(&mut self) -> LINE_DONE_INT_W<8> {
        LINE_DONE_INT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn fram_done_int(&mut self) -> FRAM_DONE_INT_W<9> {
        FRAM_DONE_INT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn te_int(&mut self) -> TE_INT_W<10> {
        TE_INT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rd_done_int(&mut self) -> RD_DONE_INT_W<11> {
        RD_DONE_INT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn timer_int(&mut self) -> TIMER_INT_W<12> {
        TIMER_INT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_fifo_full_int(&mut self) -> DBI_FIFO_FULL_INT_W<13> {
        DBI_FIFO_FULL_INT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_fifo_empty_int(&mut self) -> DBI_FIFO_EMPTY_INT_W<14> {
        DBI_FIFO_EMPTY_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_int](index.html) module"]
pub struct DBI_INT_SPEC;
impl crate::RegisterSpec for DBI_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_int::R](R) reader structure"]
impl crate::Readable for DBI_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_int::W](W) writer structure"]
impl crate::Writable for DBI_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_int to value 0"]
impl crate::Resettable for DBI_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
