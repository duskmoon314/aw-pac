#[doc = "Register `vra1speedup_ctrl` reader"]
pub type R = crate::R<VRA1SPEEDUP_CTRL_SPEC>;
#[doc = "Register `vra1speedup_ctrl` writer"]
pub type W = crate::W<VRA1SPEEDUP_CTRL_SPEC>;
#[doc = "Field `vra1speedup_rst_ctrl` reader - VAR1Speedup Down RST Manual Control Enable"]
pub type VRA1SPEEDUP_RST_CTRL_R = crate::BitReader<VRA1SPEEDUP_RST_CTRL_A>;
#[doc = "VAR1Speedup Down RST Manual Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRA1SPEEDUP_RST_CTRL_A {
    #[doc = "0: Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    DISABLED = 0,
    #[doc = "1: Enabled.\n\nVAR1Speedup Down reset 0 immediately."]
    ENABLED = 1,
}
impl From<VRA1SPEEDUP_RST_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VRA1SPEEDUP_RST_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl VRA1SPEEDUP_RST_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VRA1SPEEDUP_RST_CTRL_A {
        match self.bits {
            false => VRA1SPEEDUP_RST_CTRL_A::DISABLED,
            true => VRA1SPEEDUP_RST_CTRL_A::ENABLED,
        }
    }
    #[doc = "Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VRA1SPEEDUP_RST_CTRL_A::DISABLED
    }
    #[doc = "Enabled.\n\nVAR1Speedup Down reset 0 immediately."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VRA1SPEEDUP_RST_CTRL_A::ENABLED
    }
}
#[doc = "Field `vra1speedup_rst_ctrl` writer - VAR1Speedup Down RST Manual Control Enable"]
pub type VRA1SPEEDUP_RST_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, VRA1SPEEDUP_RST_CTRL_A>;
impl<'a, REG> VRA1SPEEDUP_RST_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VRA1SPEEDUP_RST_CTRL_A::DISABLED)
    }
    #[doc = "Enabled.\n\nVAR1Speedup Down reset 0 immediately."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VRA1SPEEDUP_RST_CTRL_A::ENABLED)
    }
}
#[doc = "Field `vra1speedup_ctrl` reader - VAR1Speedup Down Manual Control Enable"]
pub type VRA1SPEEDUP_CTRL_R = crate::BitReader<VRA1SPEEDUP_CTRL_A>;
#[doc = "VAR1Speedup Down Manual Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRA1SPEEDUP_CTRL_A {
    #[doc = "0: Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    DISABLED = 0,
    #[doc = "1: Enabled.\n\nVAR1Speedup Down converts to 1 immediately."]
    ENABLED = 1,
}
impl From<VRA1SPEEDUP_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: VRA1SPEEDUP_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl VRA1SPEEDUP_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VRA1SPEEDUP_CTRL_A {
        match self.bits {
            false => VRA1SPEEDUP_CTRL_A::DISABLED,
            true => VRA1SPEEDUP_CTRL_A::ENABLED,
        }
    }
    #[doc = "Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VRA1SPEEDUP_CTRL_A::DISABLED
    }
    #[doc = "Enabled.\n\nVAR1Speedup Down converts to 1 immediately."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VRA1SPEEDUP_CTRL_A::ENABLED
    }
}
#[doc = "Field `vra1speedup_ctrl` writer - VAR1Speedup Down Manual Control Enable"]
pub type VRA1SPEEDUP_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, VRA1SPEEDUP_CTRL_A>;
impl<'a, REG> VRA1SPEEDUP_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled.\n\nVAR1Speedup Down converts to 1 after the bus rst releases 32 ms."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VRA1SPEEDUP_CTRL_A::DISABLED)
    }
    #[doc = "Enabled.\n\nVAR1Speedup Down converts to 1 immediately."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VRA1SPEEDUP_CTRL_A::ENABLED)
    }
}
#[doc = "Field `vra1speedup_state` reader - Only if VAR1SPEEDUP_Further_CTRL (0x310\\[22\\]) is set 0, VAR1Speedup Down State is valid."]
pub type VRA1SPEEDUP_STATE_R = crate::BitReader<VRA1SPEEDUP_STATE_A>;
#[doc = "Only if VAR1SPEEDUP_Further_CTRL (0x310\\[22\\]) is set 0, VAR1Speedup Down State is valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRA1SPEEDUP_STATE_A {
    #[doc = "0: VAR1Speedup_Down does not work."]
    NOT_WORK = 0,
    #[doc = "1: VAR1Speedup_Down works."]
    WORK = 1,
}
impl From<VRA1SPEEDUP_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: VRA1SPEEDUP_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl VRA1SPEEDUP_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VRA1SPEEDUP_STATE_A {
        match self.bits {
            false => VRA1SPEEDUP_STATE_A::NOT_WORK,
            true => VRA1SPEEDUP_STATE_A::WORK,
        }
    }
    #[doc = "VAR1Speedup_Down does not work."]
    #[inline(always)]
    pub fn is_not_work(&self) -> bool {
        *self == VRA1SPEEDUP_STATE_A::NOT_WORK
    }
    #[doc = "VAR1Speedup_Down works."]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == VRA1SPEEDUP_STATE_A::WORK
    }
}
impl R {
    #[doc = "Bit 0 - VAR1Speedup Down RST Manual Control Enable"]
    #[inline(always)]
    pub fn vra1speedup_rst_ctrl(&self) -> VRA1SPEEDUP_RST_CTRL_R {
        VRA1SPEEDUP_RST_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VAR1Speedup Down Manual Control Enable"]
    #[inline(always)]
    pub fn vra1speedup_ctrl(&self) -> VRA1SPEEDUP_CTRL_R {
        VRA1SPEEDUP_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Only if VAR1SPEEDUP_Further_CTRL (0x310\\[22\\]) is set 0, VAR1Speedup Down State is valid."]
    #[inline(always)]
    pub fn vra1speedup_state(&self) -> VRA1SPEEDUP_STATE_R {
        VRA1SPEEDUP_STATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VAR1Speedup Down RST Manual Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vra1speedup_rst_ctrl(&mut self) -> VRA1SPEEDUP_RST_CTRL_W<VRA1SPEEDUP_CTRL_SPEC> {
        VRA1SPEEDUP_RST_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - VAR1Speedup Down Manual Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vra1speedup_ctrl(&mut self) -> VRA1SPEEDUP_CTRL_W<VRA1SPEEDUP_CTRL_SPEC> {
        VRA1SPEEDUP_CTRL_W::new(self, 1)
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
#[doc = "VRA1 Speedup Down Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vra1speedup_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vra1speedup_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VRA1SPEEDUP_CTRL_SPEC;
impl crate::RegisterSpec for VRA1SPEEDUP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vra1speedup_ctrl::R`](R) reader structure"]
impl crate::Readable for VRA1SPEEDUP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vra1speedup_ctrl::W`](W) writer structure"]
impl crate::Writable for VRA1SPEEDUP_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vra1speedup_ctrl to value 0"]
impl crate::Resettable for VRA1SPEEDUP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
