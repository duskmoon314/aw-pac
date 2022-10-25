#[doc = "Register `dbi_ctl_2` reader"]
pub struct R(crate::R<DBI_CTL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CTL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CTL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CTL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dbi_ctl_2` writer"]
pub struct W(crate::W<DBI_CTL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CTL_2_SPEC>;
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
impl From<crate::W<DBI_CTL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CTL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `te_en` reader - TE Enable"]
pub type TE_EN_R = crate::BitReader<bool>;
#[doc = "Field `te_en` writer - TE Enable"]
pub type TE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
#[doc = "Field `te_trig_sel` reader - TE edge trigger select"]
pub type TE_TRIG_SEL_R = crate::BitReader<bool>;
#[doc = "Field `te_trig_sel` writer - TE edge trigger select"]
pub type TE_TRIG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
#[doc = "Field `te_dbc_sel` reader - TE debounce function select"]
pub type TE_DBC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `te_dbc_sel` writer - TE debounce function select"]
pub type TE_DBC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
#[doc = "Field `dbi_sdi_sel` reader - DBI SDI PIN FUnction Select"]
pub type DBI_SDI_SEL_R = crate::FieldReader<u8, DBI_SDI_SEL_A>;
#[doc = "DBI SDI PIN FUnction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBI_SDI_SEL_A {
    #[doc = "0: `0`"]
    DBI_SDI = 0,
    #[doc = "1: `1`"]
    DBI_TE = 1,
    #[doc = "2: `10`"]
    DBI_DCX = 2,
}
impl From<DBI_SDI_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_SDI_SEL_A) -> Self {
        variant as _
    }
}
impl DBI_SDI_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBI_SDI_SEL_A> {
        match self.bits {
            0 => Some(DBI_SDI_SEL_A::DBI_SDI),
            1 => Some(DBI_SDI_SEL_A::DBI_TE),
            2 => Some(DBI_SDI_SEL_A::DBI_DCX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DBI_SDI`"]
    #[inline(always)]
    pub fn is_dbi_sdi(&self) -> bool {
        *self == DBI_SDI_SEL_A::DBI_SDI
    }
    #[doc = "Checks if the value of the field is `DBI_TE`"]
    #[inline(always)]
    pub fn is_dbi_te(&self) -> bool {
        *self == DBI_SDI_SEL_A::DBI_TE
    }
    #[doc = "Checks if the value of the field is `DBI_DCX`"]
    #[inline(always)]
    pub fn is_dbi_dcx(&self) -> bool {
        *self == DBI_SDI_SEL_A::DBI_DCX
    }
}
#[doc = "Field `dbi_sdi_sel` writer - DBI SDI PIN FUnction Select"]
pub type DBI_SDI_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_2_SPEC, u8, DBI_SDI_SEL_A, 2, O>;
impl<'a, const O: u8> DBI_SDI_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dbi_sdi(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_SDI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dbi_te(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_TE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn dbi_dcx(self) -> &'a mut W {
        self.variant(DBI_SDI_SEL_A::DBI_DCX)
    }
}
#[doc = "Field `dbi_dcx_sel` reader - DBI DCX PIN Function Select"]
pub type DBI_DCX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dbi_dcx_sel` writer - DBI DCX PIN Function Select"]
pub type DBI_DCX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
#[doc = "Field `dbi_sdq_out_sel` reader - DBI SDI PIN Output Select"]
pub type DBI_SDQ_OUT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dbi_sdq_out_sel` writer - DBI SDI PIN Output Select"]
pub type DBI_SDQ_OUT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
#[doc = "Field `dbi_trig_level` reader - DBI FIFO Empty Request Trigger Level"]
pub type DBI_TRIG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dbi_trig_level` writer - DBI FIFO Empty Request Trigger Level"]
pub type DBI_TRIG_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBI_CTL_2_SPEC, u8, u8, 7, O>;
#[doc = "Field `dbi_fifo_drq_en` reader - DBI FIFO DMA Request Enable"]
pub type DBI_FIFO_DRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `dbi_fifo_drq_en` writer - DBI FIFO DMA Request Enable"]
pub type DBI_FIFO_DRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBI_CTL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TE Enable"]
    #[inline(always)]
    pub fn te_en(&self) -> TE_EN_R {
        TE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TE edge trigger select"]
    #[inline(always)]
    pub fn te_trig_sel(&self) -> TE_TRIG_SEL_R {
        TE_TRIG_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TE debounce function select"]
    #[inline(always)]
    pub fn te_dbc_sel(&self) -> TE_DBC_SEL_R {
        TE_DBC_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DBI SDI PIN FUnction Select"]
    #[inline(always)]
    pub fn dbi_sdi_sel(&self) -> DBI_SDI_SEL_R {
        DBI_SDI_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - DBI DCX PIN Function Select"]
    #[inline(always)]
    pub fn dbi_dcx_sel(&self) -> DBI_DCX_SEL_R {
        DBI_DCX_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DBI SDI PIN Output Select"]
    #[inline(always)]
    pub fn dbi_sdq_out_sel(&self) -> DBI_SDQ_OUT_SEL_R {
        DBI_SDQ_OUT_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - DBI FIFO Empty Request Trigger Level"]
    #[inline(always)]
    pub fn dbi_trig_level(&self) -> DBI_TRIG_LEVEL_R {
        DBI_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - DBI FIFO DMA Request Enable"]
    #[inline(always)]
    pub fn dbi_fifo_drq_en(&self) -> DBI_FIFO_DRQ_EN_R {
        DBI_FIFO_DRQ_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TE Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te_en(&mut self) -> TE_EN_W<0> {
        TE_EN_W::new(self)
    }
    #[doc = "Bit 1 - TE edge trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn te_trig_sel(&mut self) -> TE_TRIG_SEL_W<1> {
        TE_TRIG_SEL_W::new(self)
    }
    #[doc = "Bit 2 - TE debounce function select"]
    #[inline(always)]
    #[must_use]
    pub fn te_dbc_sel(&mut self) -> TE_DBC_SEL_W<2> {
        TE_DBC_SEL_W::new(self)
    }
    #[doc = "Bits 3:4 - DBI SDI PIN FUnction Select"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_sdi_sel(&mut self) -> DBI_SDI_SEL_W<3> {
        DBI_SDI_SEL_W::new(self)
    }
    #[doc = "Bit 5 - DBI DCX PIN Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_dcx_sel(&mut self) -> DBI_DCX_SEL_W<5> {
        DBI_DCX_SEL_W::new(self)
    }
    #[doc = "Bit 6 - DBI SDI PIN Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_sdq_out_sel(&mut self) -> DBI_SDQ_OUT_SEL_W<6> {
        DBI_SDQ_OUT_SEL_W::new(self)
    }
    #[doc = "Bits 8:14 - DBI FIFO Empty Request Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_trig_level(&mut self) -> DBI_TRIG_LEVEL_W<8> {
        DBI_TRIG_LEVEL_W::new(self)
    }
    #[doc = "Bit 15 - DBI FIFO DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_fifo_drq_en(&mut self) -> DBI_FIFO_DRQ_EN_W<15> {
        DBI_FIFO_DRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_ctl_2](index.html) module"]
pub struct DBI_CTL_2_SPEC;
impl crate::RegisterSpec for DBI_CTL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_ctl_2::R](R) reader structure"]
impl crate::Readable for DBI_CTL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_ctl_2::W](W) writer structure"]
impl crate::Writable for DBI_CTL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_ctl_2 to value 0"]
impl crate::Resettable for DBI_CTL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
