export const NodeService = {
    getTreeTableNodesData() {
        return [
            {
                key: 't7',
                data: {
                    name: 'T7 General Power Config'
                },
                children: [
                    {
                        key: 't7-0',
                        data: {
                            name: 'IDLEACQINT',
                            value: 16
                        }
                    },
                    {
                        key: 't7-1',
                        data: {
                            name: 'ACTVACQINT',
                            value: 30
                        }
                    },
                    {
                        key: 't7-2',
                        data: {
                            name: 'ACTV2IDLETO',
                            value: 50
                        }
                    },
                    {
                        key: 't7-3',
                        data: {
                            name: 'CFG'
                        },
                        children: [
                            {
                                key: 't7-3-0',
                                data: {
                                    name: 'IDLEPIPEEN',
                                    value: 1,
                                    shift: 0,
                                    mask: 0x1
                                }
                            },
                            {
                                key: 't7-3-1',
                                data: {
                                    name: 'ACTVPIPEEN',
                                    value: 1,
                                    shift: 1,
                                    mask: 0x2
                                }
                            },
                            {
                                key: 't7-3-2',
                                data: {
                                    name: 'ACTV2IDLETOMSB',
                                    value: 0,
                                    shift: 2,
                                    mask: 0x3C
                                }
                            },
                            {
                                key: 't7-3-3',
                                data: {
                                    name: 'OVFRPTSUP',
                                    value: 0,
                                    shift: 6,
                                    mask: 0x40
                                }
                            },
                            {
                                key: 't7-3-4',
                                data: {
                                    name: 'INITACTV',
                                    value: 0,
                                    shift: 7,
                                    mask: 0x80
                                }
                            },
                        ]
                    },
                    {
                        key: 't7-4',
                        data: {
                            name: 'CFG2'
                        },
                        children: [
                            {
                                key: 't7-4-0',
                                data: {
                                    name: 'IGNSTATICTCH',
                                    value: 0,
                                    shift: 0,
                                    mask: 0x1
                                }
                            },
                            {
                                key: 't7-4-1',
                                data: {
                                    name: 'DISPOWMON',
                                    value: 0,
                                    shift: 1,
                                    mask: 0x2
                                }
                            },
                            {
                                key: 't7-4-2',
                                data: {
                                    name: 'POWMONMODE',
                                    value: 0,
                                    shift: 2,
                                    mask: 0x4
                                }
                            }
                        ]
                    },
                    {
                        key: 't7-5',
                        data: {
                            name: 'IDLEACQINTFINE',
                            value: 0
                        }
                    },
                    {
                        key: 't7-6',
                        data: {
                            name: 'ACTVACQINTFINE',
                            value: 0
                        }
                    }
                ]
            }
        ];
    },

    getTreeTableNodes() {
        return Promise.resolve(this.getTreeTableNodesData());
    }
};
