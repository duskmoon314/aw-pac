#[doc = "Register `lcd_lvds_ana%s` reader"]
pub type R = crate::R<LCD_LVDS_ANA_SPEC>;
#[doc = "Register `lcd_lvds_ana%s` writer"]
pub type W = crate::W<LCD_LVDS_ANA_SPEC>;
#[doc = "Field `lvds_plr` reader - LVDS data channel \\[3:0\\] direction."]
pub type LVDS_PLR_R = crate::FieldReader;
#[doc = "Field `lvds_plr` writer - LVDS data channel \\[3:0\\] direction."]
pub type LVDS_PLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lvds_plrc` reader - LVDS clock channel direction."]
pub type LVDS_PLRC_R = crate::BitReader<LVDS_PLRC_A>;
#[doc = "LVDS clock channel direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_PLRC_A {
    #[doc = "0: Normal"]
    NORMAL = 0,
    #[doc = "1: Reverse"]
    REVERSE = 1,
}
impl From<LVDS_PLRC_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_PLRC_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_PLRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVDS_PLRC_A {
        match self.bits {
            false => LVDS_PLRC_A::NORMAL,
            true => LVDS_PLRC_A::REVERSE,
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LVDS_PLRC_A::NORMAL
    }
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LVDS_PLRC_A::REVERSE
    }
}
#[doc = "Field `lvds_plrc` writer - LVDS clock channel direction."]
pub type LVDS_PLRC_W<'a, REG> = crate::BitWriter<'a, REG, LVDS_PLRC_A>;
impl<'a, REG> LVDS_PLRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_PLRC_A::NORMAL)
    }
    #[doc = "Reverse"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_PLRC_A::REVERSE)
    }
}
#[doc = "Field `lvds_r` reader - Adjust current flowing through R of R to change the common signals amplitude."]
pub type LVDS_R_R = crate::FieldReader<LVDS_R_A>;
#[doc = "Adjust current flowing through R of R to change the common signals amplitude.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVDS_R_A {
    #[doc = "0: 0.925 V"]
    _0_925_V = 0,
    #[doc = "1: 0.950 V"]
    _0_950_V = 1,
    #[doc = "2: 0.975 V"]
    _0_975_V = 2,
    #[doc = "3: 1.000 V"]
    _1_000_V = 3,
    #[doc = "4: 1.025 V"]
    _1_025_V = 4,
    #[doc = "5: 1.050 V"]
    _1_050_V = 5,
    #[doc = "6: 1.075 V"]
    _1_075_V = 6,
    #[doc = "7: 1.100 V"]
    _1_100_V = 7,
}
impl From<LVDS_R_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS_R_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVDS_R_A {
    type Ux = u8;
}
impl LVDS_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVDS_R_A {
        match self.bits {
            0 => LVDS_R_A::_0_925_V,
            1 => LVDS_R_A::_0_950_V,
            2 => LVDS_R_A::_0_975_V,
            3 => LVDS_R_A::_1_000_V,
            4 => LVDS_R_A::_1_025_V,
            5 => LVDS_R_A::_1_050_V,
            6 => LVDS_R_A::_1_075_V,
            7 => LVDS_R_A::_1_100_V,
            _ => unreachable!(),
        }
    }
    #[doc = "0.925 V"]
    #[inline(always)]
    pub fn is_0_925_v(&self) -> bool {
        *self == LVDS_R_A::_0_925_V
    }
    #[doc = "0.950 V"]
    #[inline(always)]
    pub fn is_0_950_v(&self) -> bool {
        *self == LVDS_R_A::_0_950_V
    }
    #[doc = "0.975 V"]
    #[inline(always)]
    pub fn is_0_975_v(&self) -> bool {
        *self == LVDS_R_A::_0_975_V
    }
    #[doc = "1.000 V"]
    #[inline(always)]
    pub fn is_1_000_v(&self) -> bool {
        *self == LVDS_R_A::_1_000_V
    }
    #[doc = "1.025 V"]
    #[inline(always)]
    pub fn is_1_025_v(&self) -> bool {
        *self == LVDS_R_A::_1_025_V
    }
    #[doc = "1.050 V"]
    #[inline(always)]
    pub fn is_1_050_v(&self) -> bool {
        *self == LVDS_R_A::_1_050_V
    }
    #[doc = "1.075 V"]
    #[inline(always)]
    pub fn is_1_075_v(&self) -> bool {
        *self == LVDS_R_A::_1_075_V
    }
    #[doc = "1.100 V"]
    #[inline(always)]
    pub fn is_1_100_v(&self) -> bool {
        *self == LVDS_R_A::_1_100_V
    }
}
#[doc = "Field `lvds_r` writer - Adjust current flowing through R of R to change the common signals amplitude."]
pub type LVDS_R_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LVDS_R_A>;
impl<'a, REG> LVDS_R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.925 V"]
    #[inline(always)]
    pub fn _0_925_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_0_925_V)
    }
    #[doc = "0.950 V"]
    #[inline(always)]
    pub fn _0_950_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_0_950_V)
    }
    #[doc = "0.975 V"]
    #[inline(always)]
    pub fn _0_975_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_0_975_V)
    }
    #[doc = "1.000 V"]
    #[inline(always)]
    pub fn _1_000_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_1_000_V)
    }
    #[doc = "1.025 V"]
    #[inline(always)]
    pub fn _1_025_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_1_025_V)
    }
    #[doc = "1.050 V"]
    #[inline(always)]
    pub fn _1_050_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_1_050_V)
    }
    #[doc = "1.075 V"]
    #[inline(always)]
    pub fn _1_075_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_1_075_V)
    }
    #[doc = "1.100 V"]
    #[inline(always)]
    pub fn _1_100_v(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_R_A::_1_100_V)
    }
}
#[doc = "Field `lvds_den` reader - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DEN_R = crate::FieldReader;
#[doc = "Field `lvds_den` writer - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lvds_denc` reader - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DENC_R = crate::BitReader;
#[doc = "Field `lvds_denc` writer - Choose data output or PLL test clock output in LVDS_tx."]
pub type LVDS_DENC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lvds_c` reader - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
pub type LVDS_C_R = crate::FieldReader<LVDS_C_A>;
#[doc = "Adjust current flowing through Rload of Rx to change the differential signals amplitude.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVDS_C_A {
    #[doc = "0: 216 mV"]
    _216MV = 0,
    #[doc = "1: 252 mV"]
    _252MV = 1,
    #[doc = "2: 276 mV"]
    _276MV = 2,
    #[doc = "3: 312 mV"]
    _312MV = 3,
    #[doc = "4: 336 mV"]
    _336MV = 4,
    #[doc = "5: 372 mV"]
    _372MV = 5,
    #[doc = "6: 395 mV"]
    _395MV = 6,
    #[doc = "7: 432 mV"]
    _432MV = 7,
}
impl From<LVDS_C_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS_C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVDS_C_A {
    type Ux = u8;
}
impl LVDS_C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVDS_C_A {
        match self.bits {
            0 => LVDS_C_A::_216MV,
            1 => LVDS_C_A::_252MV,
            2 => LVDS_C_A::_276MV,
            3 => LVDS_C_A::_312MV,
            4 => LVDS_C_A::_336MV,
            5 => LVDS_C_A::_372MV,
            6 => LVDS_C_A::_395MV,
            7 => LVDS_C_A::_432MV,
            _ => unreachable!(),
        }
    }
    #[doc = "216 mV"]
    #[inline(always)]
    pub fn is_216mv(&self) -> bool {
        *self == LVDS_C_A::_216MV
    }
    #[doc = "252 mV"]
    #[inline(always)]
    pub fn is_252mv(&self) -> bool {
        *self == LVDS_C_A::_252MV
    }
    #[doc = "276 mV"]
    #[inline(always)]
    pub fn is_276mv(&self) -> bool {
        *self == LVDS_C_A::_276MV
    }
    #[doc = "312 mV"]
    #[inline(always)]
    pub fn is_312mv(&self) -> bool {
        *self == LVDS_C_A::_312MV
    }
    #[doc = "336 mV"]
    #[inline(always)]
    pub fn is_336mv(&self) -> bool {
        *self == LVDS_C_A::_336MV
    }
    #[doc = "372 mV"]
    #[inline(always)]
    pub fn is_372mv(&self) -> bool {
        *self == LVDS_C_A::_372MV
    }
    #[doc = "395 mV"]
    #[inline(always)]
    pub fn is_395mv(&self) -> bool {
        *self == LVDS_C_A::_395MV
    }
    #[doc = "432 mV"]
    #[inline(always)]
    pub fn is_432mv(&self) -> bool {
        *self == LVDS_C_A::_432MV
    }
}
#[doc = "Field `lvds_c` writer - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
pub type LVDS_C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, LVDS_C_A>;
impl<'a, REG> LVDS_C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "216 mV"]
    #[inline(always)]
    pub fn _216mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_216MV)
    }
    #[doc = "252 mV"]
    #[inline(always)]
    pub fn _252mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_252MV)
    }
    #[doc = "276 mV"]
    #[inline(always)]
    pub fn _276mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_276MV)
    }
    #[doc = "312 mV"]
    #[inline(always)]
    pub fn _312mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_312MV)
    }
    #[doc = "336 mV"]
    #[inline(always)]
    pub fn _336mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_336MV)
    }
    #[doc = "372 mV"]
    #[inline(always)]
    pub fn _372mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_372MV)
    }
    #[doc = "395 mV"]
    #[inline(always)]
    pub fn _395mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_395MV)
    }
    #[doc = "432 mV"]
    #[inline(always)]
    pub fn _432mv(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_C_A::_432MV)
    }
}
#[doc = "Field `lvds_hpren_drv` reader - Enable data channel\\[3:0\\] drive"]
pub type LVDS_HPREN_DRV_R = crate::FieldReader;
#[doc = "Field `lvds_hpren_drv` writer - Enable data channel\\[3:0\\] drive"]
pub type LVDS_HPREN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lvds_hpren_drvc` reader - Enable clock channel drive"]
pub type LVDS_HPREN_DRVC_R = crate::BitReader<LVDS_HPREN_DRVC_A>;
#[doc = "Enable clock channel drive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_HPREN_DRVC_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LVDS_HPREN_DRVC_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_HPREN_DRVC_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_HPREN_DRVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVDS_HPREN_DRVC_A {
        match self.bits {
            false => LVDS_HPREN_DRVC_A::DISABLE,
            true => LVDS_HPREN_DRVC_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LVDS_HPREN_DRVC_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDS_HPREN_DRVC_A::ENABLE
    }
}
#[doc = "Field `lvds_hpren_drvc` writer - Enable clock channel drive"]
pub type LVDS_HPREN_DRVC_W<'a, REG> = crate::BitWriter<'a, REG, LVDS_HPREN_DRVC_A>;
impl<'a, REG> LVDS_HPREN_DRVC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_HPREN_DRVC_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_HPREN_DRVC_A::ENABLE)
    }
}
#[doc = "Field `en_24m` reader - Enable the 24M clock"]
pub type EN_24M_R = crate::BitReader;
#[doc = "Field `en_24m` writer - Enable the 24M clock"]
pub type EN_24M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_lvds` reader - Enable LVDS"]
pub type EN_LVDS_R = crate::BitReader;
#[doc = "Field `en_lvds` writer - Enable LVDS"]
pub type EN_LVDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lvds_en_mb` reader - Enable the bias circuit of the LVDS_Ana module."]
pub type LVDS_EN_MB_R = crate::BitReader<LVDS_EN_MB_A>;
#[doc = "Enable the bias circuit of the LVDS_Ana module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDS_EN_MB_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<LVDS_EN_MB_A> for bool {
    #[inline(always)]
    fn from(variant: LVDS_EN_MB_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDS_EN_MB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVDS_EN_MB_A {
        match self.bits {
            false => LVDS_EN_MB_A::DISABLE,
            true => LVDS_EN_MB_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LVDS_EN_MB_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LVDS_EN_MB_A::ENABLE
    }
}
#[doc = "Field `lvds_en_mb` writer - Enable the bias circuit of the LVDS_Ana module."]
pub type LVDS_EN_MB_W<'a, REG> = crate::BitWriter<'a, REG, LVDS_EN_MB_A>;
impl<'a, REG> LVDS_EN_MB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_EN_MB_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LVDS_EN_MB_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - LVDS data channel \\[3:0\\] direction."]
    #[inline(always)]
    pub fn lvds_plr(&self) -> LVDS_PLR_R {
        LVDS_PLR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - LVDS clock channel direction."]
    #[inline(always)]
    pub fn lvds_plrc(&self) -> LVDS_PLRC_R {
        LVDS_PLRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Adjust current flowing through R of R to change the common signals amplitude."]
    #[inline(always)]
    pub fn lvds_r(&self) -> LVDS_R_R {
        LVDS_R_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_den(&self) -> LVDS_DEN_R {
        LVDS_DEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    pub fn lvds_denc(&self) -> LVDS_DENC_R {
        LVDS_DENC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
    #[inline(always)]
    pub fn lvds_c(&self) -> LVDS_C_R {
        LVDS_C_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Enable data channel\\[3:0\\] drive"]
    #[inline(always)]
    pub fn lvds_hpren_drv(&self) -> LVDS_HPREN_DRV_R {
        LVDS_HPREN_DRV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable clock channel drive"]
    #[inline(always)]
    pub fn lvds_hpren_drvc(&self) -> LVDS_HPREN_DRVC_R {
        LVDS_HPREN_DRVC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable the 24M clock"]
    #[inline(always)]
    pub fn en_24m(&self) -> EN_24M_R {
        EN_24M_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable LVDS"]
    #[inline(always)]
    pub fn en_lvds(&self) -> EN_LVDS_R {
        EN_LVDS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the bias circuit of the LVDS_Ana module."]
    #[inline(always)]
    pub fn lvds_en_mb(&self) -> LVDS_EN_MB_R {
        LVDS_EN_MB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - LVDS data channel \\[3:0\\] direction."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_plr(&mut self) -> LVDS_PLR_W<LCD_LVDS_ANA_SPEC> {
        LVDS_PLR_W::new(self, 0)
    }
    #[doc = "Bit 4 - LVDS clock channel direction."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_plrc(&mut self) -> LVDS_PLRC_W<LCD_LVDS_ANA_SPEC> {
        LVDS_PLRC_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Adjust current flowing through R of R to change the common signals amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_r(&mut self) -> LVDS_R_W<LCD_LVDS_ANA_SPEC> {
        LVDS_R_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_den(&mut self) -> LVDS_DEN_W<LCD_LVDS_ANA_SPEC> {
        LVDS_DEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - Choose data output or PLL test clock output in LVDS_tx."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_denc(&mut self) -> LVDS_DENC_W<LCD_LVDS_ANA_SPEC> {
        LVDS_DENC_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Adjust current flowing through Rload of Rx to change the differential signals amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_c(&mut self) -> LVDS_C_W<LCD_LVDS_ANA_SPEC> {
        LVDS_C_W::new(self, 17)
    }
    #[doc = "Bits 20:23 - Enable data channel\\[3:0\\] drive"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_hpren_drv(&mut self) -> LVDS_HPREN_DRV_W<LCD_LVDS_ANA_SPEC> {
        LVDS_HPREN_DRV_W::new(self, 20)
    }
    #[doc = "Bit 24 - Enable clock channel drive"]
    #[inline(always)]
    #[must_use]
    pub fn lvds_hpren_drvc(&mut self) -> LVDS_HPREN_DRVC_W<LCD_LVDS_ANA_SPEC> {
        LVDS_HPREN_DRVC_W::new(self, 24)
    }
    #[doc = "Bit 28 - Enable the 24M clock"]
    #[inline(always)]
    #[must_use]
    pub fn en_24m(&mut self) -> EN_24M_W<LCD_LVDS_ANA_SPEC> {
        EN_24M_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn en_lvds(&mut self) -> EN_LVDS_W<LCD_LVDS_ANA_SPEC> {
        EN_LVDS_W::new(self, 29)
    }
    #[doc = "Bit 31 - Enable the bias circuit of the LVDS_Ana module."]
    #[inline(always)]
    #[must_use]
    pub fn lvds_en_mb(&mut self) -> LVDS_EN_MB_W<LCD_LVDS_ANA_SPEC> {
        LVDS_EN_MB_W::new(self, 31)
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
#[doc = "LCD LVDS Analog Register \\[i\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_lvds_ana::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_lvds_ana::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_LVDS_ANA_SPEC;
impl crate::RegisterSpec for LCD_LVDS_ANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_lvds_ana::R`](R) reader structure"]
impl crate::Readable for LCD_LVDS_ANA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_lvds_ana::W`](W) writer structure"]
impl crate::Writable for LCD_LVDS_ANA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_lvds_ana%s to value 0"]
impl crate::Resettable for LCD_LVDS_ANA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
