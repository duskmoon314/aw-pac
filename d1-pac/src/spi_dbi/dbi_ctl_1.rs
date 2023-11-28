#[doc = "Register `dbi_ctl_1` reader"]
pub type R = crate::R<DBI_CTL_1_SPEC>;
#[doc = "Register `dbi_ctl_1` writer"]
pub type W = crate::W<DBI_CTL_1_SPEC>;
#[doc = "Field `rdbn` reader - Read Data Number of Bytes"]
pub type RDBN_R = crate::FieldReader;
#[doc = "Field `rdbn` writer - Read Data Number of Bytes"]
pub type RDBN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rcdc` reader - Read Command Dummy Cycles"]
pub type RCDC_R = crate::FieldReader;
#[doc = "Field `rcdc` writer - Read Command Dummy Cycles"]
pub type RCDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rdat_lsb` reader - Bit Order of Read Data"]
pub type RDAT_LSB_R = crate::BitReader;
#[doc = "Field `rdat_lsb` writer - Bit Order of Read Data"]
pub type RDAT_LSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgb16_data_source_select` reader - RGB 16 Data Source Select"]
pub type RGB16_DATA_SOURCE_SELECT_R = crate::BitReader;
#[doc = "Field `rgb16_data_source_select` writer - RGB 16 Data Source Select"]
pub type RGB16_DATA_SOURCE_SELECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcx_data` reader - DCX Data Value"]
pub type DCX_DATA_R = crate::BitReader;
#[doc = "Field `dcx_data` writer - DCX Data Value"]
pub type DCX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbi_clko_inv` reader - DBI Clock Output Inverse"]
pub type DBI_CLKO_INV_R = crate::BitReader;
#[doc = "Field `dbi_clko_inv` writer - DBI Clock Output Inverse"]
pub type DBI_CLKO_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbi_clko_mod` reader - DBI Output Clock Mode"]
pub type DBI_CLKO_MOD_R = crate::BitReader<DBI_CLKO_MOD_A>;
#[doc = "DBI Output Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBI_CLKO_MOD_A {
    #[doc = "0: `0`"]
    ALWAYS_ON = 0,
    #[doc = "1: `1`"]
    AUTO_GATING = 1,
}
impl From<DBI_CLKO_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_CLKO_MOD_A) -> Self {
        variant as u8 != 0
    }
}
impl DBI_CLKO_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBI_CLKO_MOD_A {
        match self.bits {
            false => DBI_CLKO_MOD_A::ALWAYS_ON,
            true => DBI_CLKO_MOD_A::AUTO_GATING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == DBI_CLKO_MOD_A::ALWAYS_ON
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_auto_gating(&self) -> bool {
        *self == DBI_CLKO_MOD_A::AUTO_GATING
    }
}
#[doc = "Field `dbi_clko_mod` writer - DBI Output Clock Mode"]
pub type DBI_CLKO_MOD_W<'a, REG> = crate::BitWriter<'a, REG, DBI_CLKO_MOD_A>;
impl<'a, REG> DBI_CLKO_MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_CLKO_MOD_A::ALWAYS_ON)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn auto_gating(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_CLKO_MOD_A::AUTO_GATING)
    }
}
#[doc = "Field `dbi_rxclk_inv` reader - DBI RX Clock Inverse"]
pub type DBI_RXCLK_INV_R = crate::BitReader<DBI_RXCLK_INV_A>;
#[doc = "DBI RX Clock Inverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBI_RXCLK_INV_A {
    #[doc = "0: `0`"]
    POSITIVE = 0,
    #[doc = "1: `1`"]
    NEGATIVE = 1,
}
impl From<DBI_RXCLK_INV_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_RXCLK_INV_A) -> Self {
        variant as u8 != 0
    }
}
impl DBI_RXCLK_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBI_RXCLK_INV_A {
        match self.bits {
            false => DBI_RXCLK_INV_A::POSITIVE,
            true => DBI_RXCLK_INV_A::NEGATIVE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DBI_RXCLK_INV_A::POSITIVE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DBI_RXCLK_INV_A::NEGATIVE
    }
}
#[doc = "Field `dbi_rxclk_inv` writer - DBI RX Clock Inverse"]
pub type DBI_RXCLK_INV_W<'a, REG> = crate::BitWriter<'a, REG, DBI_RXCLK_INV_A>;
impl<'a, REG> DBI_RXCLK_INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_RXCLK_INV_A::POSITIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_RXCLK_INV_A::NEGATIVE)
    }
}
#[doc = "Field `rgb666_fmt` reader - 2 Data Lane RGB666 Format"]
pub type RGB666_FMT_R = crate::FieldReader<RGB666_FMT_A>;
#[doc = "2 Data Lane RGB666 Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB666_FMT_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SPECIAL_ILITEK = 1,
    #[doc = "2: `10`"]
    SPECIAL_NEW_VISION = 2,
}
impl From<RGB666_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB666_FMT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGB666_FMT_A {
    type Ux = u8;
}
impl RGB666_FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RGB666_FMT_A> {
        match self.bits {
            0 => Some(RGB666_FMT_A::NORMAL),
            1 => Some(RGB666_FMT_A::SPECIAL_ILITEK),
            2 => Some(RGB666_FMT_A::SPECIAL_NEW_VISION),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RGB666_FMT_A::NORMAL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_special_ilitek(&self) -> bool {
        *self == RGB666_FMT_A::SPECIAL_ILITEK
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_special_new_vision(&self) -> bool {
        *self == RGB666_FMT_A::SPECIAL_NEW_VISION
    }
}
#[doc = "Field `rgb666_fmt` writer - 2 Data Lane RGB666 Format"]
pub type RGB666_FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RGB666_FMT_A>;
impl<'a, REG> RGB666_FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RGB666_FMT_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn special_ilitek(self) -> &'a mut crate::W<REG> {
        self.variant(RGB666_FMT_A::SPECIAL_ILITEK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn special_new_vision(self) -> &'a mut crate::W<REG> {
        self.variant(RGB666_FMT_A::SPECIAL_NEW_VISION)
    }
}
#[doc = "Field `dbi_en_mode_sel` reader - DBI Enable Mode Select"]
pub type DBI_EN_MODE_SEL_R = crate::FieldReader<DBI_EN_MODE_SEL_A>;
#[doc = "DBI Enable Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBI_EN_MODE_SEL_A {
    #[doc = "0: `0`"]
    DBI = 0,
    #[doc = "1: `1`"]
    SOFTWARE = 1,
    #[doc = "2: `10`"]
    TIMER = 2,
    #[doc = "3: `11`"]
    TE = 3,
}
impl From<DBI_EN_MODE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_EN_MODE_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBI_EN_MODE_SEL_A {
    type Ux = u8;
}
impl DBI_EN_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBI_EN_MODE_SEL_A {
        match self.bits {
            0 => DBI_EN_MODE_SEL_A::DBI,
            1 => DBI_EN_MODE_SEL_A::SOFTWARE,
            2 => DBI_EN_MODE_SEL_A::TIMER,
            3 => DBI_EN_MODE_SEL_A::TE,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_dbi(&self) -> bool {
        *self == DBI_EN_MODE_SEL_A::DBI
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == DBI_EN_MODE_SEL_A::SOFTWARE
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == DBI_EN_MODE_SEL_A::TIMER
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_te(&self) -> bool {
        *self == DBI_EN_MODE_SEL_A::TE
    }
}
#[doc = "Field `dbi_en_mode_sel` writer - DBI Enable Mode Select"]
pub type DBI_EN_MODE_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DBI_EN_MODE_SEL_A>;
impl<'a, REG> DBI_EN_MODE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dbi(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_EN_MODE_SEL_A::DBI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_EN_MODE_SEL_A::SOFTWARE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_EN_MODE_SEL_A::TIMER)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn te(self) -> &'a mut crate::W<REG> {
        self.variant(DBI_EN_MODE_SEL_A::TE)
    }
}
#[doc = "Field `dbi_soft_trg` reader - DBI Soft Trigger"]
pub type DBI_SOFT_TRG_R = crate::BitReader;
#[doc = "Field `dbi_soft_trg` writer - DBI Soft Trigger"]
pub type DBI_SOFT_TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read Data Number of Bytes"]
    #[inline(always)]
    pub fn rdbn(&self) -> RDBN_R {
        RDBN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read Command Dummy Cycles"]
    #[inline(always)]
    pub fn rcdc(&self) -> RCDC_R {
        RCDC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 20 - Bit Order of Read Data"]
    #[inline(always)]
    pub fn rdat_lsb(&self) -> RDAT_LSB_R {
        RDAT_LSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RGB 16 Data Source Select"]
    #[inline(always)]
    pub fn rgb16_data_source_select(&self) -> RGB16_DATA_SOURCE_SELECT_R {
        RGB16_DATA_SOURCE_SELECT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DCX Data Value"]
    #[inline(always)]
    pub fn dcx_data(&self) -> DCX_DATA_R {
        DCX_DATA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBI Clock Output Inverse"]
    #[inline(always)]
    pub fn dbi_clko_inv(&self) -> DBI_CLKO_INV_R {
        DBI_CLKO_INV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DBI Output Clock Mode"]
    #[inline(always)]
    pub fn dbi_clko_mod(&self) -> DBI_CLKO_MOD_R {
        DBI_CLKO_MOD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DBI RX Clock Inverse"]
    #[inline(always)]
    pub fn dbi_rxclk_inv(&self) -> DBI_RXCLK_INV_R {
        DBI_RXCLK_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 2 Data Lane RGB666 Format"]
    #[inline(always)]
    pub fn rgb666_fmt(&self) -> RGB666_FMT_R {
        RGB666_FMT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 29:30 - DBI Enable Mode Select"]
    #[inline(always)]
    pub fn dbi_en_mode_sel(&self) -> DBI_EN_MODE_SEL_R {
        DBI_EN_MODE_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - DBI Soft Trigger"]
    #[inline(always)]
    pub fn dbi_soft_trg(&self) -> DBI_SOFT_TRG_R {
        DBI_SOFT_TRG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Data Number of Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rdbn(&mut self) -> RDBN_W<DBI_CTL_1_SPEC> {
        RDBN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Read Command Dummy Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn rcdc(&mut self) -> RCDC_W<DBI_CTL_1_SPEC> {
        RCDC_W::new(self, 8)
    }
    #[doc = "Bit 20 - Bit Order of Read Data"]
    #[inline(always)]
    #[must_use]
    pub fn rdat_lsb(&mut self) -> RDAT_LSB_W<DBI_CTL_1_SPEC> {
        RDAT_LSB_W::new(self, 20)
    }
    #[doc = "Bit 21 - RGB 16 Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rgb16_data_source_select(&mut self) -> RGB16_DATA_SOURCE_SELECT_W<DBI_CTL_1_SPEC> {
        RGB16_DATA_SOURCE_SELECT_W::new(self, 21)
    }
    #[doc = "Bit 22 - DCX Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn dcx_data(&mut self) -> DCX_DATA_W<DBI_CTL_1_SPEC> {
        DCX_DATA_W::new(self, 22)
    }
    #[doc = "Bit 23 - DBI Clock Output Inverse"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_clko_inv(&mut self) -> DBI_CLKO_INV_W<DBI_CTL_1_SPEC> {
        DBI_CLKO_INV_W::new(self, 23)
    }
    #[doc = "Bit 24 - DBI Output Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_clko_mod(&mut self) -> DBI_CLKO_MOD_W<DBI_CTL_1_SPEC> {
        DBI_CLKO_MOD_W::new(self, 24)
    }
    #[doc = "Bit 25 - DBI RX Clock Inverse"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_rxclk_inv(&mut self) -> DBI_RXCLK_INV_W<DBI_CTL_1_SPEC> {
        DBI_RXCLK_INV_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - 2 Data Lane RGB666 Format"]
    #[inline(always)]
    #[must_use]
    pub fn rgb666_fmt(&mut self) -> RGB666_FMT_W<DBI_CTL_1_SPEC> {
        RGB666_FMT_W::new(self, 26)
    }
    #[doc = "Bits 29:30 - DBI Enable Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_en_mode_sel(&mut self) -> DBI_EN_MODE_SEL_W<DBI_CTL_1_SPEC> {
        DBI_EN_MODE_SEL_W::new(self, 29)
    }
    #[doc = "Bit 31 - DBI Soft Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_soft_trg(&mut self) -> DBI_SOFT_TRG_W<DBI_CTL_1_SPEC> {
        DBI_SOFT_TRG_W::new(self, 31)
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
#[doc = "DBI Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_ctl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_ctl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_CTL_1_SPEC;
impl crate::RegisterSpec for DBI_CTL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_ctl_1::R`](R) reader structure"]
impl crate::Readable for DBI_CTL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_ctl_1::W`](W) writer structure"]
impl crate::Writable for DBI_CTL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dbi_ctl_1 to value 0"]
impl crate::Resettable for DBI_CTL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
