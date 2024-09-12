import { Ref, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface MapNode {
    key: string,
    data: {
        name: string,
        value: Ref
        readonly: boolean
    },
    children: (MapNode)[]
}
const registers = ref<MapNode[]>([]);

const T7Offsets : { [key: string] : number } = {
    idleacqint : 0,
    actacqint : 1,
    actv2idelto: 2,
    cfg: 3,
    cfg2: 4,
    idleacqintfine: 5,
    actvaqintfine: 6
};

const T8Offsets : { [key: string] : number } = {
    chrgtime : 0,
    reserved : -1,
    tchdrift : 2,
    driftst : 3,
    tchautocal : 4,
    sync : 5,
    atchcalst : 6,
    atchcalsthr : 7,
    atchfrccalthr : 8,
    atchfrccalratio : 9,
    measallow : 10,
    reserved2 : -1,
    cfg : 14
};

const T25Offsets : { [key: string] : number } = {
    ctrl : 0,
    cmd : 1,
    upsiglim_lsb : 2,
    upsiglim_msb : 3,
    losiglim_lsb : 4,
    losiglim_msb : 5,
    pindwellus : 6,
    sigrangelim_lsb : 7,
    sigrangelim_msb : 8,
    pinthr : 9,
    pertstinterval : 10,
    pertstholdoff : 11,
    pertstrptfactor : 12,
    pertstrtpwidth : 13,
    pertstcfg : 14,
    semeasen : 15,
    segain : 16,
    sedxgain : 17
};

const T42Offsets : { [key: string] : number } = {
    ctrl : 0,
    reserved : -1,
    maxapprarea : 2,
    maxtcharea : 3,
    supstrength : 4,
    supextto : 5,
    maxnumtchs : 6,
    shapestrength : 7,
    supdist : 8,
    disthyst : 9,
    maxscrnarea : 10,
    cfg : 11,
    reserved2 : -1,
    edgesupstrength : 13
};

const T46Offsets : { [key: string] : number } = {
    reserved : -1,
    idlesyncsperx : 2,
    activesyncsperx : 3,
    adcspersync : 4,
    pulsesperadc : 5,
    xslew : 6,
    syncdelay : -1,
    xvoltage : 9,
    reserved2 : -1,
    inrushcfg : 11,
    reserved3 : -1,
    cfg : 18
}

const T47Offsets : { [key: string] : number } = {
    ctrl : 0,
    reserved : -1,
    contmax : 2,
    stability : 3,
    maxtcharea : 4,
    amplthr : 5,
    styshape : 6,
    hoversup : 7,
    confthr : 8,
    syncsperx : 9,
    xposadj : 10,
    yposadj : 11,
    cfg : 12,
    reserved2 : -1,
    supstyto : 20,
    maxnumsty : 21,
    xedgectrl : 22,
    xedgedist : 23, // Missing from C
    yedgectrl : 24,
    yedgedist : 25, // Missing from C
    supto : 26,
    supclassmode : 27,
    dxxedgectrl : 28,
    dxxedgedist : 29,
    xedgectrlhi : 30,
    xedgedisthi : 31,
    dxxedgectrlhi : 32,
    dxxedgedisthi : 33,
    yedgectrlhi : 34,
    yedgedisthi : 35,
    cfg2 : 36,
    movfilter : 37,
    movsmooth : 38,
    movpred : 39,
    satbxlo : 40,
    satbxhi : 41,
    satbylo : 42,
    satbyhi : 43,
    satbdxxlo : 44,
    satbdxxhi : 45,
    movhistcfg : 46,
}

const T56Offsets : { [key: string] : number } = {
    ctrl: 0,
    reserved: -1,
    optint: 2,
    inttime: 3,
    intdelay : -1
}

const T65Offsets : { [key: string] : number } = {
    ctrl : 0,
    gradthr : 1, 
    ylonoisemul : -1,
    ylonoisediv : -1,
    yhinoisemul : -1,
    yhinoisediv : -1,
    lpfiltcoef : 10,
    forcescale : -1,
    forcethr : 13,
    forcethrhyst : 14,
    forcedi : 15,
    forcehyst : 16,
    atchratio : 17,
    reserved : -1,
    exfrcthr : 20,
    exfrcthrhyst : 21,
    exfrcto : 22
}

const T80Offsets : { [key: string] : number } = {
    ctrl : 0,
    compgain : 1,
    targetdelta : 2,
    compthr : 3,
    atchthr : 4,
    moistcfg : 5,
    reserved : -1,
    moistthr : 7,
    moistinvtchthr : 8,
    moistcfg2 : 9,
    compstrthr : 10,
    compcfg : 11,
    moistvldthrsf : 12,
    moistcfg3 : 13,
    moistdegthr : 14
}

const T100Offsets : { [key: string] : number } = {
    ctrl : 0,
    cfg1 : 1,
    scraux : 2,
    tchaux : 3,
    tcheventcfg : 4,
    akscfg : 5,
    numtch : 6,
    xycfg : 7,
    xorigin : 8,
    xsize : 9,
    xpitch : 10,
    xlocip : 11,
    xhiclip : 12,
    xrange  : -1,
    xedgecfg : 15,
    xedgedist : 16,
    dxxedgecfg : 17,
    dxxedgedist : 18,
    yorigin : 19,
    ysize : 20,
    ypitch : 21,
    ylocip : 22,
    yhiclip : 23,
    yrange  : -1,
    yedgecfg : 26,
    yedgedist : 27,
    gain : 28,
    dxgain : 29,
    tchthr : 30,
    tchhyst : 31,
    intthr : 32,
    noisesf : 33,
    cutoffthr : 34,
    mrgthr : 35,
    mrgthradjstr : 36,
    mrghyst : 37,
    dxthrsf : 38,
    tchdidown : 39,
    tchdiup : 40,
    nexttchdi : 41,
    calcfg : 42,
    jumplimit : 43,
    movfilter : 44,
    movsmooth : 45,
    movpred : 46,
    movhysti  : -1,
    movhystn  : -1,
    amplhyst : 51,
    scrareahyst : 52,
    intthryst : 53,
    xedgecfghi : 54,
    xedgedisthi : 55,
    dxxedgecfghi : 56,
    dxxedgedisthi : 57,
    yedgecfghi : 58,
    yedgedisthi : 59,
    cfg2 : 60,
    movhystcfg : 61,
    amplcoeff : 62,
    amploffset : 63,
    jumplimitmov : 64,
    jlmmovthr  : -1,
    jlmmovintth : 67
}

function readObject(id : number, name : string, offsets : { [key: string] : number }) {
    (invoke("read_object", { id: id }) as Promise<string>).then((obj_str) => {
        let obj = JSON.parse(obj_str);
        let obj_node : MapNode = {
            key: 't'+id,
            data: {
                name: name,
                value: ref(),
                readonly: true
            },
            children: []
        };
        for (const register in obj) {
            let value = ref(obj[register]);
            let child : MapNode = { key: register, data: { name: register, value: value, readonly: offsets[register] < 0 }, children: [] };
            watch (value, async (newValue) => {
                // TODO: Handle bitfields and u16
                invoke("write_register", {id: id, offset: offsets[register], data: [newValue]}).then(() => {
                }).catch((e) => {
                    console.log(e);
                });
            });
            obj_node.children.push(child);
        }
        registers.value.push(obj_node);
    });
}

export const NodeService = {
    update() {
        registers.value = [];
        readObject(7, 'T7 General Power Config', T7Offsets);
        readObject(8, 'T8 Acquisition Config', T8Offsets);
        readObject(25, 'T25 Self Test', T25Offsets);
        readObject(42, 'T42 Touch Suppression', T42Offsets);
        readObject(46, 'T46 CTE Config', T46Offsets);
        readObject(47, 'T47 Passive Stylus Config', T47Offsets);
        readObject(56, 'T56 Shieldless Config', T56Offsets);
        readObject(65, 'T65 Lens Bending Config', T65Offsets);
        readObject(80, 'T80 Retransmission Compensation', T80Offsets);
        readObject(100, 'T100 Multiple Touch Touchscreen', T100Offsets);
    },
    getTreeTableNodesData() {
        return registers.value;
    },

    getTreeTableNodes() {
        return Promise.resolve(this.getTreeTableNodesData());
    }
};
