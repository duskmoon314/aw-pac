#[doc = "Register `asrcmbistcfg` reader"]
pub type R = crate::R<ASRCMBISTCFG_SPEC>;
#[doc = "Register `asrcmbistcfg` writer"]
pub type W = crate::W<ASRCMBISTCFG_SPEC>;
#[doc = "Field `asrc_rom_bist_en` reader - ASRC ROM BIST Enable"]
pub type ASRC_ROM_BIST_EN_R = crate::BitReader<ASRC_ROM_BIST_EN_A>;
#[doc = "ASRC ROM BIST Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_ROM_BIST_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_ROM_BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_ROM_BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_ROM_BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASRC_ROM_BIST_EN_A {
        match self.bits {
            false => ASRC_ROM_BIST_EN_A::DISABLE,
            true => ASRC_ROM_BIST_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_ROM_BIST_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_ROM_BIST_EN_A::ENABLE
    }
}
#[doc = "Field `asrc_rom_bist_en` writer - ASRC ROM BIST Enable"]
pub type ASRC_ROM_BIST_EN_W<'a, REG> = crate::BitWriter<'a, REG, ASRC_ROM_BIST_EN_A>;
impl<'a, REG> ASRC_ROM_BIST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_ROM_BIST_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_ROM_BIST_EN_A::ENABLE)
    }
}
#[doc = "Field `asrc_ram_bist_en` reader - ASTC RAM BIST Enable"]
pub type ASRC_RAM_BIST_EN_R = crate::BitReader<ASRC_RAM_BIST_EN_A>;
#[doc = "ASTC RAM BIST Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_RAM_BIST_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_RAM_BIST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_RAM_BIST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_RAM_BIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASRC_RAM_BIST_EN_A {
        match self.bits {
            false => ASRC_RAM_BIST_EN_A::DISABLE,
            true => ASRC_RAM_BIST_EN_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_RAM_BIST_EN_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_RAM_BIST_EN_A::ENABLE
    }
}
#[doc = "Field `asrc_ram_bist_en` writer - ASTC RAM BIST Enable"]
pub type ASRC_RAM_BIST_EN_W<'a, REG> = crate::BitWriter<'a, REG, ASRC_RAM_BIST_EN_A>;
impl<'a, REG> ASRC_RAM_BIST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_RAM_BIST_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ASRC_RAM_BIST_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - ASRC ROM BIST Enable"]
    #[inline(always)]
    pub fn asrc_rom_bist_en(&self) -> ASRC_ROM_BIST_EN_R {
        ASRC_ROM_BIST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - ASTC RAM BIST Enable"]
    #[inline(always)]
    pub fn asrc_ram_bist_en(&self) -> ASRC_RAM_BIST_EN_R {
        ASRC_RAM_BIST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ASRC ROM BIST Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_rom_bist_en(&mut self) -> ASRC_ROM_BIST_EN_W<ASRCMBISTCFG_SPEC> {
        ASRC_ROM_BIST_EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - ASTC RAM BIST Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_ram_bist_en(&mut self) -> ASRC_RAM_BIST_EN_W<ASRCMBISTCFG_SPEC> {
        ASRC_RAM_BIST_EN_W::new(self, 8)
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
#[doc = "ASRC MBIST Test Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asrcmbistcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asrcmbistcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRCMBISTCFG_SPEC;
impl crate::RegisterSpec for ASRCMBISTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrcmbistcfg::R`](R) reader structure"]
impl crate::Readable for ASRCMBISTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrcmbistcfg::W`](W) writer structure"]
impl crate::Writable for ASRCMBISTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcmbistcfg to value 0"]
impl crate::Resettable for ASRCMBISTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
