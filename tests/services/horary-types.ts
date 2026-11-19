import {
  calculate, calculateFromLocal, calculateWithContext, buildChartModel, renderChartSvg,
  type HoraryBodyId, type HoraryHouseOrder, type HoraryMoment, type HoraryRequest, type HoraryResult,
} from '../../bindings/node';

const moment: HoraryMoment = {
  utc: { year: 2026, month: 11, day: 19, hour: 8, minute: 0 },
  location: { latitude: 10.8231, longitude: 106.6297 },
  houseSystem: 'regiomontanus',
};
const request: HoraryRequest = { operation: 'horary', moment, quesitedHouse: 7 };
const direct: HoraryResult = calculate(request);
const contextual: HoraryResult = calculateWithContext({ ...request, ruleset: 'western-traditional-v1' }).result;
const local = calculateFromLocal({
  operation: 'horary', quesitedHouse: 7,
  moment: { location: moment.location, local: { ...moment.utc, hour: 15 }, timeZone: 'Asia/Ho_Chi_Minh' },
});
const ruler: HoraryBodyId = local.data.significators.quesited.rulerBodyId;
const course = direct.data.voidOfCourseMoon;
if (course.isVoidOfCourse) {
  const noContact: null = course.nextAspect;
  void noContact;
} else {
  const contact: Exclude<HoraryBodyId, 'moon'> = course.nextAspect.otherBodyId;
  void contact;
}
const svg: string = renderChartSvg(direct, { chart: 'horary', focus: { house: 7, layer: 'Q' } });
const order: HoraryHouseOrder = direct.calculation.houseOrder;
const width: number | undefined = buildChartModel(direct, { chart: 'horary' }).houses[0].signedWidth;
void [contextual, ruler, local.calculation.timeResolution.moment.utc, svg, order, width];

// @ts-expect-error The released ruleset vocabulary is explicit.
calculate({ ...request, ruleset: 'future-method' });
// @ts-expect-error Horary-specific house systems do not widen natal inputs.
calculate({ operation: 'natal', utc: moment.utc, location: moment.location, houseSystem: 'regiomontanus' });
// @ts-expect-error The caller must select the house of the question.
calculate({ operation: 'horary', moment });
