use crate::prelude::*;
use mutagen::Reborrow;

pub struct ProtoUpdArg<'a> {
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ProtoUpdArg<'a>> for ProtoUpdArg<'b> {
    fn reborrow(&'a mut self) -> ProtoUpdArg<'a> {
        ProtoUpdArg {
            profiler: &mut self.profiler,
        }
    }
}

impl<'a> mutagen::State for ProtoUpdArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

pub struct ProtoGenArg<'a> {
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ProtoGenArg<'a>> for ProtoGenArg<'b> {
    fn reborrow(&'a mut self) -> ProtoGenArg<'a> {
        ProtoGenArg {
            profiler: &mut self.profiler,
        }
    }
}

impl<'a> mutagen::State for ProtoGenArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

pub struct ProtoMutArg<'a> {
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ProtoMutArg<'a>> for ProtoMutArg<'b> {
    fn reborrow(&'a mut self) -> ProtoMutArg<'a> {
        ProtoMutArg {
            profiler: &mut self.profiler,
        }
    }
}

impl<'a> mutagen::State for ProtoMutArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> From<ProtoMutArg<'a>> for ProtoGenArg<'a> {
    fn from(arg: ProtoMutArg<'a>) -> ProtoGenArg {
        ProtoGenArg {
            profiler: arg.profiler,
        }
    }
}
