import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'coordinate-system',
    {
      type: 'category',
      label: 'Tattvas',
      link: {type: 'doc', id: 'tattvas/index'},
      items: [
        'tattvas/primitives',
        'tattvas/text',
        'tattvas/composite',
        'tattvas/graphs',
        'tattvas/math',
        'tattvas/layout',
        'tattvas/storytelling',
        'tattvas/utility',
      ],
    },
    'animations',
    'scene-and-app',
    'camera',
    'updaters',
    'roadmap',
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'architecture/architecture',
        'architecture/scene-timeline',
        'architecture/end-to-end-flow',
        'architecture/tattva',
        'architecture/dirty-flags',
        'architecture/ecs',
        'architecture/renderer',
        'architecture/projection',
        'architecture/text-and-latex',
      ],
    },

  ],
};

export default sidebars;
