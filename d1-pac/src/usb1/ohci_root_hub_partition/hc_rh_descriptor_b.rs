#[doc = "Register `hc_rh_descriptor_b` reader"]
pub type R = crate::R<HC_RH_DESCRIPTOR_B_SPEC>;
#[doc = "Register `hc_rh_descriptor_b` writer"]
pub type W = crate::W<HC_RH_DESCRIPTOR_B_SPEC>;
#[doc = "Field `device_removable` reader - DeviceRemovable\n\nEach bit is dedicated to a port of the Root Hub. When cleared, the attached device is removable. When set, the attached device is not removable."]
pub type DEVICE_REMOVABLE_R = crate::FieldReader<u16>;
#[doc = "Field `device_removable` writer - DeviceRemovable\n\nEach bit is dedicated to a port of the Root Hub. When cleared, the attached device is removable. When set, the attached device is not removable."]
pub type DEVICE_REMOVABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `port_power_control_mask` reader - PortPowerControlMask\n\nEach bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set. When set, the port's power state is only affected by per-port power control ( Set/ClearPortPower ). When cleared, the port is controlled by the global power switch ( Set/ClearGlobalPower ). If the device is configured to global switching mode ( PowerSwitchingMode = 0 ), this field is not valid."]
pub type PORT_POWER_CONTROL_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `port_power_control_mask` writer - PortPowerControlMask\n\nEach bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set. When set, the port's power state is only affected by per-port power control ( Set/ClearPortPower ). When cleared, the port is controlled by the global power switch ( Set/ClearGlobalPower ). If the device is configured to global switching mode ( PowerSwitchingMode = 0 ), this field is not valid."]
pub type PORT_POWER_CONTROL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DeviceRemovable\n\nEach bit is dedicated to a port of the Root Hub. When cleared, the attached device is removable. When set, the attached device is not removable."]
    #[inline(always)]
    pub fn device_removable(&self) -> DEVICE_REMOVABLE_R {
        DEVICE_REMOVABLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PortPowerControlMask\n\nEach bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set. When set, the port's power state is only affected by per-port power control ( Set/ClearPortPower ). When cleared, the port is controlled by the global power switch ( Set/ClearGlobalPower ). If the device is configured to global switching mode ( PowerSwitchingMode = 0 ), this field is not valid."]
    #[inline(always)]
    pub fn port_power_control_mask(&self) -> PORT_POWER_CONTROL_MASK_R {
        PORT_POWER_CONTROL_MASK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DeviceRemovable\n\nEach bit is dedicated to a port of the Root Hub. When cleared, the attached device is removable. When set, the attached device is not removable."]
    #[inline(always)]
    #[must_use]
    pub fn device_removable(&mut self) -> DEVICE_REMOVABLE_W<HC_RH_DESCRIPTOR_B_SPEC> {
        DEVICE_REMOVABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - PortPowerControlMask\n\nEach bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set. When set, the port's power state is only affected by per-port power control ( Set/ClearPortPower ). When cleared, the port is controlled by the global power switch ( Set/ClearGlobalPower ). If the device is configured to global switching mode ( PowerSwitchingMode = 0 ), this field is not valid."]
    #[inline(always)]
    #[must_use]
    pub fn port_power_control_mask(
        &mut self,
    ) -> PORT_POWER_CONTROL_MASK_W<HC_RH_DESCRIPTOR_B_SPEC> {
        PORT_POWER_CONTROL_MASK_W::new(self, 16)
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
#[doc = "OHCI Root Hub Descriptor Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_rh_descriptor_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_rh_descriptor_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_RH_DESCRIPTOR_B_SPEC;
impl crate::RegisterSpec for HC_RH_DESCRIPTOR_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_rh_descriptor_b::R`](R) reader structure"]
impl crate::Readable for HC_RH_DESCRIPTOR_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_rh_descriptor_b::W`](W) writer structure"]
impl crate::Writable for HC_RH_DESCRIPTOR_B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_rh_descriptor_b to value 0"]
impl crate::Resettable for HC_RH_DESCRIPTOR_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
